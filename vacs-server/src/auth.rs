use crate::config::AuthConfig;
use crate::ws::message::{receive_message, send_message, MessageResult};
use axum::extract::ws;
use axum::extract::ws::WebSocket;
use futures_util::stream::{SplitSink, SplitStream};
use std::time::Duration;
use vacs_protocol::{LoginFailureReason, SignalingMessage};

pub async fn verify_token(_client_id: &str, token: &str) -> anyhow::Result<()> {
    tracing::trace!("Verifying auth token");

    // TODO actual token verification
    if token.is_empty() {
        return Err(anyhow::anyhow!("Invalid token"));
    }

    Ok(())
}

pub async fn handle_login(
    auth_config: &AuthConfig,
    websocket_receiver: &mut SplitStream<WebSocket>,
    websocket_sender: &mut SplitSink<WebSocket, ws::Message>,
) -> Option<String> {
    tracing::trace!("Handling login flow");
    match tokio::time::timeout(Duration::from_millis(auth_config.login_flow_timeout_millis), async {
        loop {
            return match receive_message(websocket_receiver).await {
                MessageResult::ApplicationMessage(SignalingMessage::Login { id, token }) => {
                    if verify_token(&id, &token).await.is_err() {
                        let login_failure_message = SignalingMessage::LoginFailure {
                            reason: LoginFailureReason::InvalidCredentials,
                        };
                        if let Err(err) =
                            send_message(websocket_sender, login_failure_message).await
                        {
                            tracing::warn!(?err, "Failed to send login failure message");
                        }
                        return None;
                    }
                    tracing::trace!("Login flow completed");
                    Some(id)
                }
                MessageResult::ApplicationMessage(message) => {
                    tracing::debug!(msg = ?message, "Received unexpected message during login flow");
                    let login_failure_message = SignalingMessage::LoginFailure {
                        reason: LoginFailureReason::Unauthorized,
                    };
                    if let Err(err) = send_message(websocket_sender, login_failure_message).await {
                        tracing::warn!(?err, "Failed to send login failure message");
                    }
                    None
                }
                MessageResult::ControlMessage => {
                    tracing::trace!("Skipping control message during login");
                    continue;
                }
                MessageResult::Disconnected => {
                    tracing::debug!("Client disconnected during login flow");
                    None
                }
                MessageResult::Error(err) => {
                    tracing::warn!(?err, "Received error while handling login flow");
                    None
                }
            };
        }
    }).await {
        Ok(Some(id)) => Some(id),
        Ok(None) => None,
        Err(_) => {
            tracing::debug!("Login flow timed out");
            let login_timeout_message = SignalingMessage::LoginFailure {
                reason: LoginFailureReason::Timeout,
            };
            if let Err(err) = send_message(websocket_sender, login_timeout_message).await {
                tracing::warn!(?err, "Failed to send login timeout message");
            }
            None
        }
    }
}
