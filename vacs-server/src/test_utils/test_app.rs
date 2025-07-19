use crate::app::{create_app, serve};
use crate::config::{AppConfig, AuthConfig};
use crate::session::setup_memory_session_manager;
use crate::state::AppState;
use std::sync::Arc;
use tokio::sync::watch;
use tokio::task::JoinHandle;

pub struct TestApp {
    state: Arc<AppState>,
    addr: String,
    shutdown_tx: watch::Sender<()>,
    handle: JoinHandle<()>,
}

impl TestApp {
    pub async fn new() -> Self {
        let config = AppConfig {
            auth: AuthConfig {
                login_flow_timeout_millis: 100,
                ..Default::default()
            },
            ..Default::default()
        };

        let (shutdown_tx, shutdown_rx) = watch::channel(());
        let state = Arc::new(AppState::new(
            config.clone(),
            shutdown_rx,
        ));

        let session_layer = setup_memory_session_manager(&config).await.unwrap();
        let app = create_app(session_layer);
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();

        let state_clone = state.clone();
        let handle = tokio::spawn(async move {
            serve(listener, app, state_clone).await;
        });

        Self {
            state,
            addr: format!("ws://{addr}/ws"),
            shutdown_tx,
            handle,
        }
    }

    pub fn addr(&self) -> &str {
        &self.addr
    }

    pub fn state(&self) -> Arc<AppState> {
        self.state.clone()
    }
}

impl Drop for TestApp {
    fn drop(&mut self) {
        self.shutdown_tx.send(()).unwrap();
        self.handle.abort();
    }
}
