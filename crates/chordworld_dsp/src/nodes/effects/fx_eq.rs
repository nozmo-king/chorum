//! 3-band parametric EQ

use crate::{AudioBusMut, AudioBusRef, DspNode, ParamSpec, PortSpec, ProcessCtx};
use chordworld_core::{EventQueue, ParamIndex, ParamValue};
use std::f32::consts::PI;

static PORTS: &[PortSpec] = &[PortSpec::audio_in("in", 1), PortSpec::audio_out("out", 1)];
static PARAMS: &[ParamSpec] = &[
    ParamSpec::new("low_freq", 100.0, 20.0, 500.0, "Hz"),
    ParamSpec::new("low_gain", 0.0, -12.0, 12.0, "dB"),
    ParamSpec::new("mid_freq", 1000.0, 200.0, 5000.0, "Hz"),
    ParamSpec::new("mid_gain", 0.0, -12.0, 12.0, "dB"),
    ParamSpec::new("mid_q", 1.0, 0.1, 10.0, ""),
    ParamSpec::new("high_freq", 5000.0, 2000.0, 20000.0, "Hz"),
    ParamSpec::new("high_gain", 0.0, -12.0, 12.0, "dB"),
];

struct BiquadState { x1: f32, x2: f32, y1: f32, y2: f32 }
impl BiquadState {
    fn new() -> Self { Self { x1: 0.0, x2: 0.0, y1: 0.0, y2: 0.0 } }
    fn process(&mut self, input: f32, b0: f32, b1: f32, b2: f32, a1: f32, a2: f32) -> f32 {
        let y = b0 * input + b1 * self.x1 + b2 * self.x2 - a1 * self.y1 - a2 * self.y2;
        self.x2 = self.x1; self.x1 = input; self.y2 = self.y1; self.y1 = y;
        y
    }
}

pub struct FxEQ {
    low_freq: f32, low_gain: f32,
    mid_freq: f32, mid_gain: f32, mid_q: f32,
    high_freq: f32, high_gain: f32,
    low_state: BiquadState, mid_state: BiquadState, high_state: BiquadState,
    sample_rate: f32,
}

impl FxEQ {
    pub fn new() -> Self {
        Self {
            low_freq: 100.0, low_gain: 0.0, mid_freq: 1000.0, mid_gain: 0.0, mid_q: 1.0, high_freq: 5000.0, high_gain: 0.0,
            low_state: BiquadState::new(), mid_state: BiquadState::new(), high_state: BiquadState::new(), sample_rate: 48000.0,
        }
    }
    fn db_to_a(db: f32) -> f32 { 10.0_f32.powf(db / 40.0) }
    fn process_sample(&mut self, input: f32) -> f32 {
        // Low shelf
        let a_low = Self::db_to_a(self.low_gain);
        let w0_low = 2.0 * PI * self.low_freq / self.sample_rate;
        let cos_w0 = w0_low.cos(); let sin_w0 = w0_low.sin();
        let alpha = sin_w0 / 2.0 * ((a_low + 1.0/a_low) * (1.0/0.9 - 1.0) + 2.0).sqrt();
        let b0 = a_low * ((a_low + 1.0) - (a_low - 1.0) * cos_w0 + 2.0 * a_low.sqrt() * alpha);
        let b1 = 2.0 * a_low * ((a_low - 1.0) - (a_low + 1.0) * cos_w0);
        let b2 = a_low * ((a_low + 1.0) - (a_low - 1.0) * cos_w0 - 2.0 * a_low.sqrt() * alpha);
        let a0 = (a_low + 1.0) + (a_low - 1.0) * cos_w0 + 2.0 * a_low.sqrt() * alpha;
        let a1 = -2.0 * ((a_low - 1.0) + (a_low + 1.0) * cos_w0);
        let a2 = (a_low + 1.0) + (a_low - 1.0) * cos_w0 - 2.0 * a_low.sqrt() * alpha;
        let out = self.low_state.process(input, b0/a0, b1/a0, b2/a0, a1/a0, a2/a0);

        // Mid peaking
        let a_mid = Self::db_to_a(self.mid_gain);
        let w0_mid = 2.0 * PI * self.mid_freq / self.sample_rate;
        let alpha_mid = w0_mid.sin() / (2.0 * self.mid_q);
        let b0 = 1.0 + alpha_mid * a_mid; let b1 = -2.0 * w0_mid.cos(); let b2 = 1.0 - alpha_mid * a_mid;
        let a0 = 1.0 + alpha_mid / a_mid; let a1 = -2.0 * w0_mid.cos(); let a2 = 1.0 - alpha_mid / a_mid;
        let out = self.mid_state.process(out, b0/a0, b1/a0, b2/a0, a1/a0, a2/a0);

        // High shelf
        let a_high = Self::db_to_a(self.high_gain);
        let w0_high = 2.0 * PI * self.high_freq / self.sample_rate;
        let cos_w0 = w0_high.cos(); let sin_w0 = w0_high.sin();
        let alpha = sin_w0 / 2.0 * ((a_high + 1.0/a_high) * (1.0/0.9 - 1.0) + 2.0).sqrt();
        let b0 = a_high * ((a_high + 1.0) + (a_high - 1.0) * cos_w0 + 2.0 * a_high.sqrt() * alpha);
        let b1 = -2.0 * a_high * ((a_high - 1.0) + (a_high + 1.0) * cos_w0);
        let b2 = a_high * ((a_high + 1.0) + (a_high - 1.0) * cos_w0 - 2.0 * a_high.sqrt() * alpha);
        let a0 = (a_high + 1.0) - (a_high - 1.0) * cos_w0 + 2.0 * a_high.sqrt() * alpha;
        let a1 = 2.0 * ((a_high - 1.0) - (a_high + 1.0) * cos_w0);
        let a2 = (a_high + 1.0) - (a_high - 1.0) * cos_w0 - 2.0 * a_high.sqrt() * alpha;
        self.high_state.process(out, b0/a0, b1/a0, b2/a0, a1/a0, a2/a0)
    }
}
impl Default for FxEQ { fn default() -> Self { Self::new() } }

impl DspNode for FxEQ {
    fn reset(&mut self, sr: f32) { self.sample_rate = sr; self.low_state = BiquadState::new(); self.mid_state = BiquadState::new(); self.high_state = BiquadState::new(); }
    fn process(&mut self, _: &ProcessCtx, ins: &[AudioBusRef], outs: &mut [AudioBusMut]) {
        if ins.is_empty() || outs.is_empty() { return; }
        if let (Some(i), Some(o)) = (ins[0].channel(0), outs[0].channel(0)) {
            for (inp, outp) in i.iter().zip(o.iter_mut()) { *outp = self.process_sample(*inp); }
        }
    }
    fn set_param(&mut self, p: ParamIndex, v: ParamValue) {
        match p.0 {
            0 => self.low_freq = v.as_float().clamp(20.0, 500.0),
            1 => self.low_gain = v.as_float().clamp(-12.0, 12.0),
            2 => self.mid_freq = v.as_float().clamp(200.0, 5000.0),
            3 => self.mid_gain = v.as_float().clamp(-12.0, 12.0),
            4 => self.mid_q = v.as_float().clamp(0.1, 10.0),
            5 => self.high_freq = v.as_float().clamp(2000.0, 20000.0),
            6 => self.high_gain = v.as_float().clamp(-12.0, 12.0),
            _ => {}
        }
    }
    fn handle_events(&mut self, _: &ProcessCtx, _: &mut EventQueue) {}
}
impl crate::NodeDescriptor for FxEQ {
    fn type_name(&self) -> &'static str { "FxEQ" }
    fn ports(&self) -> &'static [PortSpec] { PORTS }
    fn params(&self) -> &'static [ParamSpec] { PARAMS }
    fn display_name(&self) -> &'static str { "3-Band EQ" }
    fn category(&self) -> &'static str { "Effects" }
}
