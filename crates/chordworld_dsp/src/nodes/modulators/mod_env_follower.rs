//! Envelope follower

use crate::{AudioBusMut, AudioBusRef, DspNode, ParamSpec, PortSpec, ProcessCtx};
use chordworld_core::{EventQueue, ParamIndex, ParamValue};

static PORTS: &[PortSpec] = &[PortSpec::audio_in("in", 1), PortSpec::audio_out("out", 1)];
static PARAMS: &[ParamSpec] = &[
    ParamSpec::new("attack", 0.01, 0.001, 0.5, "s"),
    ParamSpec::new("release", 0.1, 0.01, 2.0, "s"),
    ParamSpec::new("gain", 1.0, 0.1, 10.0, ""),
];

pub struct ModEnvFollower {
    attack: f32, release: f32, gain: f32,
    envelope: f32, sample_rate: f32,
}

impl ModEnvFollower {
    pub fn new() -> Self { Self { attack: 0.01, release: 0.1, gain: 1.0, envelope: 0.0, sample_rate: 48000.0 } }
}
impl Default for ModEnvFollower { fn default() -> Self { Self::new() } }

impl DspNode for ModEnvFollower {
    fn reset(&mut self, sr: f32) { self.sample_rate = sr; self.envelope = 0.0; }
    fn process(&mut self, _: &ProcessCtx, ins: &[AudioBusRef], outs: &mut [AudioBusMut]) {
        if ins.is_empty() || outs.is_empty() { return; }
        let attack_coeff = (-1.0 / (self.attack * self.sample_rate)).exp();
        let release_coeff = (-1.0 / (self.release * self.sample_rate)).exp();
        if let (Some(i), Some(o)) = (ins[0].channel(0), outs[0].channel(0)) {
            for (inp, outp) in i.iter().zip(o.iter_mut()) {
                let input_level = inp.abs() * self.gain;
                let coeff = if input_level > self.envelope { attack_coeff } else { release_coeff };
                self.envelope = input_level + coeff * (self.envelope - input_level);
                *outp = self.envelope;
            }
        }
    }
    fn set_param(&mut self, p: ParamIndex, v: ParamValue) {
        match p.0 { 0 => self.attack = v.as_float().clamp(0.001, 0.5), 1 => self.release = v.as_float().clamp(0.01, 2.0), 2 => self.gain = v.as_float().clamp(0.1, 10.0), _ => {} }
    }
    fn handle_events(&mut self, _: &ProcessCtx, _: &mut EventQueue) {}
}
impl crate::NodeDescriptor for ModEnvFollower {
    fn type_name(&self) -> &'static str { "ModEnvFollower" }
    fn ports(&self) -> &'static [PortSpec] { PORTS }
    fn params(&self) -> &'static [ParamSpec] { PARAMS }
    fn display_name(&self) -> &'static str { "Envelope Follower" }
    fn category(&self) -> &'static str { "Modulators" }
}
