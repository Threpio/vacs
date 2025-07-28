pub(crate) mod commands;

use tauri::{AppHandle, Emitter};
use tokio::sync::broadcast::Receiver;
use tokio::sync::watch;
use vacs_protocol::ws::{ClientInfo, SignalingMessage};
use vacs_signaling::client::SignalingClient;
use vacs_signaling::error::SignalingError;
use vacs_signaling::transport;
use crate::error::Error;

pub struct Connection {
    client: SignalingClient<transport::tokio::TokioTransport>,
    shutdown_tx: watch::Sender<()>,
    shutdown_rx: watch::Receiver<()>,
}

impl Connection {
    pub async fn new(ws_url: &str) -> Result<Self, SignalingError> {
        let (shutdown_tx, shutdown_rx) = watch::channel(());
        let transport = transport::tokio::TokioTransport::new(ws_url).await?;
        let client = SignalingClient::new(transport, shutdown_rx.clone());

        Ok(Self {
            client,
            shutdown_tx,
            shutdown_rx,
        })
    }

    pub async fn login(&mut self, token: &str)  -> Result<Vec<ClientInfo>, SignalingError> {
        self.client.login(token).await
    }

    pub async fn disconnect(&mut self) -> Result<(), Error> {
        self.shutdown_tx.send(()).map_err(|err| Error::Other(anyhow::anyhow!(err)))?;
        self.client.disconnect().await?;
        Ok(())
    }

    pub fn subscribe(&self) -> (Receiver<SignalingMessage>, watch::Receiver<()>)  {
        (self.client.subscribe(), self.shutdown_rx.clone())
    }

    pub async fn handle_interaction(mut broadcast_rx: Receiver<SignalingMessage>, mut shutdown_rx: watch::Receiver<()>, app: &AppHandle) -> Result<(), Error> {
        loop {
            tokio::select! {
                biased;

                _ = shutdown_rx.changed() => {
                    log::debug!("Shutdown signal received, aborting interaction handling");
                    break;
                }

                msg = broadcast_rx.recv() => {
                    log::trace!("Received message: {:?}", msg);
                    match msg {
                        Ok(msg) => {
                            match msg {
                                SignalingMessage::CallOffer { .. } => {}
                                SignalingMessage::CallEnd { .. } => {}
                                SignalingMessage::CallIceCandidate { .. } => {}
                                SignalingMessage::ClientConnected { client } => {
                                    log::trace!("Client connected: {:?}", client);
                                    app.emit("signaling:client-connected", client).ok();
                                }
                                SignalingMessage::ClientDisconnected { id } => {
                                    log::trace!("Client disconnected: {:?}", id);
                                    app.emit("signaling:client-disconnected", id).ok();
                                }
                                SignalingMessage::Error { .. } => {}
                                _ => {}
                            }
                        },
                        Err(_) => {
                            return Err(Error::Signaling(SignalingError::Disconnected));
                        }
                    }
                }
            }
        }

        log::debug!("Interaction handling complete");
        Ok(())
    }
}
