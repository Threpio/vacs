use crate::app::state::AppState;
use crate::error::Error;
use tauri::{AppHandle, Manager};

#[tauri::command]
#[vacs_macros::log_err]
pub async fn signaling_connect(app: AppHandle) -> Result<(), Error> {
    app.state::<AppState>().lock().await.connect(&app).await
}

#[tauri::command]
#[vacs_macros::log_err]
pub async fn signaling_disconnect(app: AppHandle) -> Result<(), Error> {
    app.state::<AppState>().lock().await.disconnect(&app).await
}