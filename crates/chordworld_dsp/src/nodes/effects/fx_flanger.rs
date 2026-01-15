//! Flanger effect

use crate::{AudioBusMut, AudioBusRef, DspNode, ParamSpec, PortSpec, ProcessCtx};
use chordworld_core::{EventQueue, ParamIndex, ParamValue};
use std::f32::consts::TAU;

const MAX_DELAY: usize = 2400; // 50ms

static PORTS: &[PortSpec] = &[PortSpec::audio_in("in", 1), PortSpec::audio_out("out", 1)];
static PARAMS: &[ParamSpec] = &[
    ParamSpec::new("rate", 0.2, 0.01, 5.0, "Hz"),
    ParamSpec::new("depth", 0.7, 0.0, 1.0, ""),
    ParamSpec::new("feedback", 0.5, -0.95, 0.95, ""),
    ParamSpec::new("mix", 0.5, 0.0, 1.0, ""),
];

pub struct FxFlanger {
    rate: f32, depth: f32, feedback: f32, mix: f32,
    buffer: Vec<f32>, write_pos: usize,
    lfo_phase: f32, sample_rate: f32,
}

impl FxFlanger {
    pub fn new() -> Self {
        Self { rate: 0.2, depth: 0.7, feedback: 0.5, mix: 0.5, buffer: vec![0.0; MAX_DELAY], write_pos: 0, lfo_phase: 0.0, sample_rate: 48000.0 }
    }
    fn process_sample(&mut self, input: f32) -> f32 {
        let lfo = (self.lfo_phase * TAU).sin() * 0.5 + 0.5;
        let delay_ms = 1.0 + lfo * self.depth * 10.0;
        let delay_samples = delay_ms * self.sample_rate / 1000.0;
        let read_pos = (self.write_pos as f32 + MAX_DELAY as f32 - delay_samples) % MAX_DELAY as f32;
        let idx0 = read_pos as usize % MAX_DELAY;
        let idx1 = (idx0 + 1) % MAX_DELAY;
        let frac = read_pos.fract();
        let delayed = self.buffer[idx0] * (1.0 - frac) + self.buffer[idx1] * frac;
        self.buffer[self.write_pos] = input + delayed * self.feedback;
        self.write_pos = (self.write_pos + 1) % MAX_DELAY;
        self.lfo_phase += self.rate / self.sample_rate;
        if self.lfo_phase >= 1.0 { self.lfo_phase -= 1.0; }
        input * (1.0 - self.mix) + delayed * self.mix
    }
}
impl Default for FxFlanger { fn default() -> Self { Self::new() } }

impl DspNode for FxFlanger {
    fn reset(&mut self, sr: f32) { self.sample_rate = sr; self.buffer.fill(0.0); self.write_pos = 0; self.lfo_phase = 0.0; }
    fn process(&mut self, _: &ProcessCtx, ins: &[AudioBusRef], outs: &mut [AudioBusMut]) {
        if ins.is_empty() || outs.is_empty() { return; }
        if let (Some(i), Some(o)) = (ins[0].channel(0), outs[0].channel(0)) {
            for (inp, outp) in i.iter().zip(o.iter_mut()) { *outp = self.process_sample(*inp); }
        }
    }
    fn set_param(&mut self, p: ParamIndex, v: ParamValue) {
        match p.0 { 0 => self.rate = v.as_float().clamp(0.01, 5.0), 1 => self.depth = v.as_float().clamp(0.0, 1.0), 2 => self.feedback = v.as_float().clamp(-0.95, 0.95), 3 => self.mix = v.as_float().clamp(0.0, 1.0), _ => {} }
    }
    fn handle_events(&mut self, _: &ProcessCtx, _: &mut EventQueue) {}
}
impl crate::NodeDescriptor for FxFlanger {
    fn type_name(&self) -> &'static str { "FxFlanger" }
    fn ports(&self) -> &'static [PortSpec] { PORTS }
    fn params(&self) -> &'static [ParamSpec] { PARAMS }
    fn display_name(&self) -> &'static str { "Flanger" }
    fn category(&self) -> &'static str { "Effects" }
}
