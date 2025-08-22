use crate::sources::AudioSource;
use crate::{EncodedAudioFrame, FRAME_SIZE, SAMPLE_RATE};
use anyhow::{Context, Result};
use ringbuf::traits::{Consumer, Producer, Split};
use ringbuf::{HeapCons, HeapProd, HeapRb};
use tokio::sync::mpsc;
use tokio::task::JoinHandle;
use tracing::{instrument, Instrument};

pub struct OpusSource {
    cons: HeapCons<f32>,
    decoder_handle: JoinHandle<()>,
    output_channels: usize, // >= 1
    volume: f32,            // 0.0 - 1.0
    amp: f32,               // >= 0.1
}

impl OpusSource {
    #[instrument(level = "debug", skip(rx))]
    pub fn from_mpsc(
        mut rx: mpsc::Receiver<EncodedAudioFrame>,
        sample_rate: u32,
        rb_capacity_samples: usize,
        output_channels: usize,
        volume: f32,
        amp: f32,
    ) -> Result<Self> {
        tracing::trace!("Creating Opus source");
        let rb: HeapRb<f32> = HeapRb::new(rb_capacity_samples);
        let (mut prod, cons): (HeapProd<f32>, HeapCons<f32>) = rb.split();

        // Our captured input audio will always be in mono and is transmitted via a webrtc mono stream,
        // so we can safely default to a mono Opus decoder here. Interleaving to stereo output devices
        // is handled by `AudioSource` implementation.
        let mut decoder = opus::Decoder::new(sample_rate, opus::Channels::Mono)
            .context("Failed to create Opus decoder")?;

        let decoder_handle = tokio::spawn(
            async move {
                tracing::debug!("Starting Opus decoder task");

                let mut decoded = vec![0.0f32; FRAME_SIZE];
                let mut overflows = 0usize;

                while let Some(frame) = rx.recv().await {
                    match decoder.decode_float(&frame, &mut decoded, false) {
                        Ok(n) => {
                            let written = prod.push_slice(&decoded[..n]);
                            if written <= n {
                                overflows += 1;
                                if overflows % 100 == 1 {
                                    tracing::debug!(
                                        ?written,
                                        needed = ?n,
                                        "Opus ring overflow (tail samples dropped)"
                                    );
                                }
                            }
                        }
                        Err(err) => {
                            tracing::error!(?err, "Failed to decode Opus frame");
                        }
                    }
                }

                tracing::debug!("Opus decoder task ended");
            }
            .instrument(tracing::Span::current()),
        );

        Ok(Self {
            cons,
            decoder_handle,
            output_channels: output_channels.max(1),
            volume: volume.clamp(0.0, 1.0),
            amp: amp.min(0.1),
        })
    }

    #[instrument(level = "trace", skip(self))]
    pub fn with_output_channels(mut self, output_channels: usize) -> Self {
        tracing::trace!(?output_channels, "Setting output channels for Opus source");
        self.output_channels = output_channels.max(1);
        self
    }

    #[instrument(level = "debug", skip(self))]
    pub fn stop(self) {
        tracing::trace!("Aborting Opus decoder task");
        self.decoder_handle.abort();
    }
}

impl From<mpsc::Receiver<EncodedAudioFrame>> for OpusSource {
    fn from(rx: mpsc::Receiver<EncodedAudioFrame>) -> Self {
        // We buffer 10 frames, which equals a total buffer of 200 ms at 48_000 Hz and 20 ms intervals
        // Default to mono output as it's the safest choice for the most devices. Interleaving to
        // multi-channel output devices can be enabled by calling `with_output_channels`.
        Self::from_mpsc(rx, SAMPLE_RATE, FRAME_SIZE * 10, 1, 0.5, 2.0).unwrap()
    }
}

impl AudioSource for OpusSource {
    fn mix_into(&mut self, output: &mut [f32]) {
        // Only a single output channel --> no interleaving required, just copy samples
        if self.output_channels == 1 {
            for (out_s, s) in output.iter_mut().zip(self.cons.pop_iter()) {
                *out_s += s * self.amp * self.volume;
            }

            // Do not backfill tail samples, as output buffer is already initialized with EQUILIBRIUM
            // and other AudioSources might have already added their samples to the buffer.
            return;
        }

        // Interleaved multi-channel: duplicate mono sample across channels
        // Limit by frames so we don’t overrun the output
        for (frame, s) in output
            .chunks_mut(self.output_channels)
            .zip(self.cons.pop_iter())
        {
            for x in frame {
                *x += s * self.amp * self.volume;
            }
        }
    }

    fn start(&mut self) {
        // Nothing to do here, the webrtc source must start webrtc stream used as opus input data
    }

    fn stop(&mut self) {
        // Nothing to do here, the webrtc source must stop webrtc stream used as opus input data
    }

    fn set_volume(&mut self, volume: f32) {
        self.volume = volume.clamp(0.0, 1.0);
    }
}
