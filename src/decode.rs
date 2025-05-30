use std::{
    fs::File,
    io::{Read, Cursor},
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};
use anyhow::{anyhow, Context};
use cpal::StreamConfig;
use cpal::traits::{HostTrait, DeviceTrait, StreamTrait};

pub fn decode() -> anyhow::Result<()> {
    // 1. Open your raw‐Opus file, which must have been written like:
    //    [u16 BE length][n bytes of Opus data][u16][data]...
    let mut file = File::open("mic_encoded.opus")?;

    // 2. Set up the Opus decoder
    let mut decoder = opus::Decoder::new(48000, opus::Channels::Mono).context("Failed to create opus encoder")?;

    // 3. Prepare a shared PCM buffer for the CPAL callback
    let shared_buf = Arc::new(Mutex::new(Vec::<f32>::new()));
    let shared_buf_cb = shared_buf.clone();

    // 4. Spawn a thread to read & decode from the file
    thread::spawn(move || -> anyhow::Result<()> {
        let mut len_buf = [0u8; 2];
        loop {
            // Read the 2-byte length prefix
            if file.read_exact(&mut len_buf).is_err() {
                break; // EOF
            }
            let frame_len = u16::from_be_bytes(len_buf) as usize;

            // Read that many bytes of encoded Opus data
            let mut enc_frame = vec![0u8; frame_len];
            file.read_exact(&mut enc_frame)?;

            // Decode to f32 PCM
            let mut pcm_out = vec![0f32; 960]; // 20 ms @ 48 kHz mono
            let decoded_samples = decoder.decode_float(&enc_frame, &mut pcm_out, false)?;

            // Push into our shared buffer
            let mut buf = shared_buf.lock().unwrap();
            buf.extend_from_slice(&pcm_out[..decoded_samples]);

            // Throttle so we don’t spin too fast
            thread::sleep(Duration::from_millis(20));
        }
        Ok(())
    });

    // 5. Set up CPAL output stream
    let host = cpal::default_host();
    let output_device = Arc::new(
        host.default_output_device()
            .context("Failed to get output device")?,
    );
    let supported_output_config = output_device
        .supported_output_configs()?
        .filter(|c| c.sample_format() == cpal::SampleFormat::F32) // or whatever you support
        .find(|c| c.min_sample_rate().0 <= 48000 && c.max_sample_rate().0 >= 48000)
        .ok_or_else(|| anyhow!("No supported output config with 48000 Hz"))?;

    let output_device_config: Arc<StreamConfig> = Arc::new(
        supported_output_config
            .with_sample_rate(cpal::SampleRate(48000))
            .into(),
    );

    let err_fn = |err| eprintln!("CPAL error: {}", err);
    let stream = output_device.build_output_stream(
        &output_device_config,
        move |output: &mut [f32], _| {
            let mut buf = shared_buf_cb.lock().unwrap();
            for sample in output.iter_mut() {
                if !buf.is_empty() {
                    *sample = buf.remove(0); // take from the front, FIFO order
                } else {
                    *sample = 0.0;
                }
            }

        },
        err_fn,
        None
    )?;

    // 6. Play!
    stream.play()?;
    // Keep the main thread alive while playback runs
    loop { thread::sleep(Duration::from_secs(1)); }
}
