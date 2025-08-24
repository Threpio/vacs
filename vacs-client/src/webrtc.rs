use crate::app::state::AppState;
use crate::error::Error;
use anyhow::Context;
use std::collections::HashMap;
use serde_json::Value;
use tauri::{AppHandle, Emitter, Manager};
use tokio::sync::mpsc;
use tokio::task::JoinHandle;
use vacs_audio::EncodedAudioFrame;
use vacs_protocol::ws::SignalingMessage;
use vacs_webrtc::config::WebrtcConfig;
use vacs_webrtc::{Peer, PeerConnectionState, PeerEvent};

const ENCODED_AUDIO_FRAME_BUFFER_SIZE: usize = 512;

struct Call {
    peer_id: String,
    peer: Peer,
    events_task: JoinHandle<()>,
}

#[derive(Default)]
pub struct WebrtcManager {
    active_call: Option<Call>,
    held_calls: HashMap<String, Call>,
    config: WebrtcConfig,
}

impl WebrtcManager {
    pub fn new(config: WebrtcConfig) -> Self {
        Self {
            config,
            ..Default::default()
        }
    }

    pub async fn start_call(&mut self, app: AppHandle, peer_id: String) -> Result<String, Error> {
        if self.active_call.is_some() {
            return Err(anyhow::anyhow!("Another call is already active").into());
        }

        let (peer, mut events_rx) = Peer::new(self.config.clone())
            .await
            .context("Failed to create WebRTC peer")?;

        let sdp = peer
            .create_offer()
            .await
            .context("Failed to create WebRTC offer")?;

        let peer_id_clone = peer_id.clone();

        let events_task = tokio::runtime::Handle::current().spawn(async move {
            loop {
                match events_rx.recv().await {
                    Ok(peer_event) => match peer_event {
                        PeerEvent::ConnectionState(state) => match state {
                            PeerConnectionState::Connected => {
                                log::info!("Connected to peer");

                                let state = app.state::<AppState>();
                                let mut state = state.lock().await;
                                let audio_config = state.config.audio.clone();

                                let (output_tx, output_rx) =
                                    mpsc::channel(ENCODED_AUDIO_FRAME_BUFFER_SIZE);
                                let (input_tx, input_rx) =
                                    mpsc::channel(ENCODED_AUDIO_FRAME_BUFFER_SIZE);

                                log::debug!("Starting peer in WebRTC manager");
                                if let Err(err) = state.webrtc_manager().start_peer(input_rx, output_tx).await {
                                    log::warn!("Failed to start peer in WebRTC manager: {err:?}");
                                    // TODO cleanup

                                    continue;
                                }

                                log::debug!("Attaching call to audio manager");
                                if let Err(err) = state.audio_manager().attach_call_output(
                                    output_rx,
                                    audio_config.output_device_volume,
                                    audio_config.output_device_volume_amp,
                                ) {
                                    log::warn!("Failed to attach call to audio manager: {err:?}");
                                    // TODO cleanup
                                    continue;
                                }

                                log::debug!("Attaching input device to audio manager");
                                if let Err(err) = state
                                    .audio_manager()
                                    .attach_input_device(&audio_config, input_tx)
                                {
                                    log::warn!(
                                        "Failed to attach input device to audio manager: {err:?}"
                                    );
                                    // TODO cleanup
                                }

                                log::info!("Successfully established call to peer");
                                app.emit("webrtc-connected", Value::Null).ok(); // TODO event?
                            }
                            PeerConnectionState::Disconnected => {
                                log::info!("Disconnected from peer");

                                let state = app.state::<AppState>();
                                let mut state = state.lock().await;

                                if state.webrtc_manager().is_active_call(&peer_id_clone)
                                {
                                    log::debug!("Detaching active call from audio manager");
                                    state.audio_manager().detach_call_output();
                                    state.audio_manager().detach_input_device();
                                    if let Some(mut call) = state.webrtc_manager().active_call.take() {
                                        log::debug!("Closing WebRTC peer");
                                        if let Err(err) = call.peer.close().await {
                                            log::warn!("Failed to close peer: {err:?}");
                                        }
                                    }
                                } else {
                                    log::debug!("Peer disconnected is not the active call, checking held calls");
                                    if let Some(mut call) = state.webrtc_manager().held_calls.remove(&peer_id_clone) {
                                        log::debug!("Closing held WebRTC peer");
                                        if let Err(err) = call.peer.close().await {
                                            log::warn!("Failed to close held WebRTC peer: {err:?}");
                                        }
                                    } else {
                                        log::debug!("Peer disconnected is not held, ignoring");
                                    }
                                }
                            }
                            PeerConnectionState::Failed => {
                                log::info!("Failed to connect to peer");
                                // Failed to connect to peer
                                // TODO display error and cleanup
                            }
                            PeerConnectionState::Closed => {
                                // Graceful close
                                log::info!("Peer closed connection");

                                let state = app.state::<AppState>();
                                let mut state = state.lock().await;

                                if state.webrtc_manager().is_active_call(&peer_id_clone)
                                {
                                    log::debug!("Detaching active call from audio manager");
                                    state.audio_manager().detach_call_output();
                                    state.audio_manager().detach_input_device();
                                    if let Some(mut call) = state.webrtc_manager().active_call.take() {
                                        log::debug!("Closing WebRTC peer");
                                        if let Err(err) = call.peer.close().await {
                                            log::warn!("Failed to close peer: {err:?}");
                                        }
                                    }
                                } else {
                                    log::debug!("Closed peer is not the active call, checking held calls");
                                    if let Some(mut call) = state.webrtc_manager().held_calls.remove(&peer_id_clone) {
                                        log::debug!("Closing held WebRTC peer");
                                        if let Err(err) = call.peer.close().await {
                                            log::warn!("Failed to close held WebRTC peer: {err:?}");
                                        }
                                    } else {
                                        log::debug!("Closed peer is not held, ignoring");
                                    }
                                }
                            }
                            state => {
                                log::trace!("Received connection state: {state:?}");
                            }
                        },
                        PeerEvent::IceCandidate(candidate) => {
                            let app_state = app.state::<AppState>();
                            if let Err(err) = app_state
                                .lock()
                                .await
                                .send_signaling_message(SignalingMessage::CallIceCandidate {
                                    peer_id: peer_id_clone.clone(),
                                    candidate,
                                })
                                .await
                            {
                                log::warn!("Failed to send ICE candidate: {err:?}");
                            }
                        }
                        PeerEvent::Error(err) => {
                            log::warn!("Received error peer event: {err}");
                        }
                    },
                    Err(err) => {
                        log::warn!("Failed to receive peer event: {err:?}");
                    }
                }
            }
        });

        self.active_call = Some(Call {
            peer_id,
            peer,
            events_task,
        });

        Ok(sdp)
    }

    pub async fn end_call(&mut self, peer_id: &str) -> bool {
        let res = if let Some(call) = &mut self.active_call
            && call.peer_id == peer_id
        {
            let result = call.peer.close().await;;
            self.active_call = None;
            result
        } else if let Some(mut call) = self.held_calls.remove(peer_id) {
            call.peer.close().await
        } else {
            Err(anyhow::anyhow!("Unknown peer {peer_id}"))
        };

        if let Err(err) = res {
            log::warn!("Failed to end call: {err:?}");
            return false;
        }

        true
    }

    pub async fn cleanup_call(&mut self, peer_id: &str) {
        if state.webrtc_manager().is_active_call(peer_id) {
            state.audio_manager().detach_call_output();
            state.audio_manager().detach_input_device();
        }

        self.end_call(peer_id).await;
    }

    async fn start_peer(
        &mut self,
        input_rx: mpsc::Receiver<EncodedAudioFrame>,
        output_tx: mpsc::Sender<EncodedAudioFrame>,
    ) -> Result<(), Error> {
        if let Some(call) = &mut self.active_call {
            call.peer
                .start(input_rx, output_tx)
                .await
                .map_err(|err| err.into())
        } else {
            log::warn!("Tried to start peer without an active call");
            Ok(())
        }
    }

    pub fn active_call_peer_id(&self) -> Option<&String> {
        self.active_call.as_ref().map(|call| &call.peer_id)
    }

    pub fn is_active_call(&self, peer_id: &str) -> bool {
        self.active_call
            .as_ref()
            .map(|call| call.peer_id == peer_id)
            .unwrap_or(false)
    }

    pub async fn add_remote_ice_candidate(&self, peer_id: &str, candidate: String) {
        let res = if let Some(call) = &self.active_call
            && call.peer_id == peer_id
        {
            call.peer.add_remote_ice_candidate(candidate).await
        } else if let Some(call) = self.held_calls.get(peer_id) {
            call.peer.add_remote_ice_candidate(candidate).await
        } else {
            Err(anyhow::anyhow!("Unknown peer {peer_id}"))
        };

        if let Err(err) = res {
            log::warn!("Failed to add remote ICE candidate: {err:?}");
        }
    }
}
