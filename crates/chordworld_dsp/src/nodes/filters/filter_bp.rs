//! Bandpass filter (two-pole)

use crate::{AudioBusMut, AudioBusRef, DspNode, ParamSpec, PortSpec, ProcessCtx};
use chordworld_core::{EventQueue, ParamIndex, ParamValue};
use std::f32::consts::PI;

static PORTS: &[PortSpec] = &[PortSpec::audio_in("in", 1), PortSpec::audio_out("out", 1)];
static PARAMS: &[ParamSpec] = &[
    ParamSpec::new("freq", 1000.0, 20.0, 20000.0, "Hz"),
    ParamSpec::new("q", 1.0, 0.1, 20.0, ""),
];

pub struct FilterBP { freq: f32, q: f32, sample_rate: f32, z1: f32, z2: f32 }

impl FilterBP {
    pub fn new() -> Self { Self { freq: 1000.0, q: 1.0, sample_rate: 48000.0, z1: 0.0, z2: 0.0 } }
}
impl Default for FilterBP { fn default() -> Self { Self::new() } }

impl DspNode for FilterBP {
    fn reset(&mut self, sr: f32) { self.sample_rate = sr; self.z1 = 0.0; self.z2 = 0.0; }
    fn process(&mut self, _ctx: &ProcessCtx, ins: &[AudioBusRef], outs: &mut [AudioBusMut]) {
        if ins.is_empty() || outs.is_empty() { return; }
        let w0 = 2.0 * PI * self.freq / self.sample_rate;
        let alpha = w0.sin() / (2.0 * self.q);
        let b0 = alpha; let b1 = 0.0; let b2 = -alpha;
        let a0 = 1.0 + alpha; let a1 = -2.0 * w0.cos(); let a2 = 1.0 - alpha;
        if let (Some(i), Some(o)) = (ins[0].channel(0), outs[0].channel(0)) {
            for (inp, outp) in i.iter().zip(o.iter_mut()) {
                let x = *inp;
                let y = (b0/a0)*x + (b1/a0)*self.z1 + (b2/a0)*self.z2 - (a1/a0)*self.z1 - (a2/a0)*self.z2;
                self.z2 = self.z1; self.z1 = x;
                *outp = y;
            }
        }
    }
    fn set_param(&mut self, p: ParamIndex, v: ParamValue) {
        match p.0 { 0 => self.freq = v.as_float().clamp(20.0, 20000.0), 1 => self.q = v.as_float().clamp(0.1, 20.0), _ => {} }
    }
    fn handle_events(&mut self, _: &ProcessCtx, _: &mut EventQueue) {}
}
impl crate::NodeDescriptor for FilterBP {
    fn type_name(&self) -> &'static str { "FilterBP" }
    fn ports(&self) -> &'static [PortSpec] { PORTS }
    fn params(&self) -> &'static [ParamSpec] { PARAMS }
    fn display_name(&self) -> &'static str { "Bandpass Filter" }
    fn category(&self) -> &'static str { "Filters" }
}
