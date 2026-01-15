//! Phaser effect

use crate::{AudioBusMut, AudioBusRef, DspNode, ParamSpec, PortSpec, ProcessCtx};
use chordworld_core::{EventQueue, ParamIndex, ParamValue};
use std::f32::consts::{PI, TAU};

const NUM_STAGES: usize = 6;

static PORTS: &[PortSpec] = &[PortSpec::audio_in("in", 1), PortSpec::audio_out("out", 1)];
static PARAMS: &[ParamSpec] = &[
    ParamSpec::new("rate", 0.3, 0.01, 5.0, "Hz"),
    ParamSpec::new("depth", 0.7, 0.0, 1.0, ""),
    ParamSpec::new("feedback", 0.5, 0.0, 0.95, ""),
    ParamSpec::new("mix", 0.5, 0.0, 1.0, ""),
];

pub struct FxPhaser {
    rate: f32, depth: f32, feedback: f32, mix: f32,
    allpass_state: [f32; NUM_STAGES],
    lfo_phase: f32, last_out: f32, sample_rate: f32,
}

impl FxPhaser {
    pub fn new() -> Self {
        Self { rate: 0.3, depth: 0.7, feedback: 0.5, mix: 0.5, allpass_state: [0.0; NUM_STAGES], lfo_phase: 0.0, last_out: 0.0, sample_rate: 48000.0 }
    }
    fn allpass(&mut self, input: f32, coeff: f32, stage: usize) -> f32 {
        let y = -coeff * input + self.allpass_state[stage];
        self.allpass_state[stage] = y * coeff + input;
        y
    }
    fn process_sample(&mut self, input: f32) -> f32 {
        let lfo = (self.lfo_phase * TAU).sin() * 0.5 + 0.5;
        let min_freq = 200.0; let max_freq = 4000.0;
        let freq = min_freq + lfo * self.depth * (max_freq - min_freq);
        let coeff = (PI * freq / self.sample_rate).tan();
        let coeff = (coeff - 1.0) / (coeff + 1.0);
        let mut out = input + self.last_out * self.feedback;
        for i in 0..NUM_STAGES { out = self.allpass(out, coeff, i); }
        self.last_out = out;
        self.lfo_phase += self.rate / self.sample_rate;
        if self.lfo_phase >= 1.0 { self.lfo_phase -= 1.0; }
        input * (1.0 - self.mix) + out * self.mix
    }
}
impl Default for FxPhaser { fn default() -> Self { Self::new() } }

impl DspNode for FxPhaser {
    fn reset(&mut self, sr: f32) { self.sample_rate = sr; self.allpass_state = [0.0; NUM_STAGES]; self.lfo_phase = 0.0; self.last_out = 0.0; }
    fn process(&mut self, _: &ProcessCtx, ins: &[AudioBusRef], outs: &mut [AudioBusMut]) {
        if ins.is_empty() || outs.is_empty() { return; }
        if let (Some(i), Some(o)) = (ins[0].channel(0), outs[0].channel(0)) {
            for (inp, outp) in i.iter().zip(o.iter_mut()) { *outp = self.process_sample(*inp); }
        }
    }
    fn set_param(&mut self, p: ParamIndex, v: ParamValue) {
        match p.0 { 0 => self.rate = v.as_float().clamp(0.01, 5.0), 1 => self.depth = v.as_float().clamp(0.0, 1.0), 2 => self.feedback = v.as_float().clamp(0.0, 0.95), 3 => self.mix = v.as_float().clamp(0.0, 1.0), _ => {} }
    }
    fn handle_events(&mut self, _: &ProcessCtx, _: &mut EventQueue) {}
}
impl crate::NodeDescriptor for FxPhaser {
    fn type_name(&self) -> &'static str { "FxPhaser" }
    fn ports(&self) -> &'static [PortSpec] { PORTS }
    fn params(&self) -> &'static [ParamSpec] { PARAMS }
    fn display_name(&self) -> &'static str { "Phaser" }
    fn category(&self) -> &'static str { "Effects" }
}
