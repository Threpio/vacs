use crate::TARGET_SAMPLE_RATE;
use biquad::{Biquad, Coefficients, DirectForm2Transposed, ToHertz, Type, Q_BUTTERWORTH_F32};

pub fn downmix_interleaved_to_mono(interleaved: &[f32], channels: usize, mono: &mut Vec<f32>) {
    debug_assert!(channels > 0);
    debug_assert_eq!(interleaved.len() % channels, 0);

    let frames = interleaved.len() / channels;
    mono.clear();
    mono.reserve(frames);
    for frame in interleaved.chunks(channels) {
        mono.push(downmix_frame_to_mono(frame));
    }
}

#[inline]
fn downmix_frame_to_mono(frame: &[f32]) -> f32 {
    match frame.len() {
        0 => 0.0f32,
        1 => frame[0],
        2 => {
            let (l, r) = (frame[0], frame[1]);
            if (l - r).abs() < 1e-4 {
                l
            } else {
                (l + r) * 0.5f32
            }
        }
        n => frame.iter().take(n).copied().sum::<f32>() / (n as f32),
    }
}

struct DcBlock {
    x1: f32,
    y1: f32,
    r: f32,
}

impl Default for DcBlock {
    fn default() -> Self {
        Self {
            x1: 0.0f32,
            y1: 0.0f32,
            r: 0.995f32,
        }
    }
}

impl DcBlock {
    #[inline]
    pub fn process(&mut self, x: f32) -> f32 {
        let y = x - self.x1 + self.r * self.y1;
        self.x1 = x;
        self.y1 = y;
        y
    }
}

struct NoiseGate {
    open_lin: f32,
    close_lin: f32,
    att_s: f32,
    rel_s: f32,
    fs: f32,
    gain: f32,
    target: f32,
}

impl Default for NoiseGate {
    fn default() -> Self {
        let lin = |db| 10.0f32.powf(db / 20.0f32);
        Self {
            open_lin: lin(-45.0f32),
            close_lin: lin(-50.0f32),
            att_s: 0.008f32, // 8ms
            rel_s: 0.09f32,  // 90ms
            fs: TARGET_SAMPLE_RATE as f32,
            gain: 0.0f32,
            target: 0.0f32,
        }
    }
}

impl NoiseGate {
    #[inline]
    fn coeff(&self, faster: bool) -> f32 {
        let tau = if faster { self.att_s } else { self.rel_s };
        let denom = (tau * self.fs).max(1e-6); // avoid div-by-zero
        1.0 - (-1.0 / denom).exp() // always in (0,1)
    }

    pub fn process_frame(&mut self, frame: &mut [f32]) {
        let mut sum = 0.0f32;
        for &s in frame.iter() {
            sum += s * s;
        }
        let rms = (sum / frame.len() as f32).sqrt();

        if rms >= self.open_lin {
            self.target = 1.0f32;
        } else if rms <= self.close_lin {
            self.target = 0.0f32;
        }

        let a = self.coeff(true);
        let r = self.coeff(false);

        for s in frame.iter_mut() {
            let c = if self.target > self.gain { a } else { r };
            self.gain += c * (self.target - self.gain);
            *s *= self.gain;
        }
    }
}

struct SoftLimiter {
    thr: f32,
}

impl Default for SoftLimiter {
    fn default() -> Self {
        Self {
            thr: 10.0f32.powf(-1.0f32 / 20.0f32),
        }
    }
}

impl SoftLimiter {
    #[inline]
    pub fn process_frame(&mut self, frame: &mut [f32]) {
        for s in frame.iter_mut() {
            let a = s.abs();
            if a > self.thr {
                let sign = s.signum();
                let over = (a - self.thr) / (1.0f32 - self.thr + 1e-9f32);
                let soft = self.thr + over / (1.0f32 + over);
                *s = sign * soft.min(0.9999f32);
            }
        }
    }
}

pub struct MicProcessor {
    dc_block: DcBlock,
    hpf: DirectForm2Transposed<f32>,
    noise_gate: NoiseGate,
    soft_limiter: SoftLimiter,
}

impl Default for MicProcessor {
    fn default() -> Self {
        let coeffs = Coefficients::from_params(
            Type::HighPass,
            TARGET_SAMPLE_RATE.hz(),
            100.hz(),
            Q_BUTTERWORTH_F32,
        )
        .expect("Failed to create HPF coefficients");
        Self {
            dc_block: DcBlock::default(),
            hpf: DirectForm2Transposed::new(coeffs),
            noise_gate: NoiseGate::default(),
            soft_limiter: SoftLimiter::default(),
        }
    }
}

impl MicProcessor {
    pub fn process_10ms(&mut self, frame: &mut [f32]) {
        for s in frame.iter_mut() {
            *s = self.dc_block.process(*s);
            *s = self.hpf.run(*s);
        }
        self.noise_gate.process_frame(frame);
        self.soft_limiter.process_frame(frame);
    }
}
