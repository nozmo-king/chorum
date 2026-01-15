//! Simple highpass filter (one-pole)

use crate::{AudioBusMut, AudioBusRef, DspNode, ParamSpec, PortSpec, ProcessCtx};
use chordworld_core::{EventQueue, ParamIndex, ParamValue};
use std::f32::consts::PI;

static PORTS: &[PortSpec] = &[PortSpec::audio_in("in", 1), PortSpec::audio_out("out", 1)];
static PARAMS: &[ParamSpec] = &[ParamSpec::new("cutoff", 1000.0, 20.0, 20000.0, "Hz")];

pub struct FilterHP { cutoff: f32, sample_rate: f32, z1: f32, prev_in: f32 }

impl FilterHP {
    pub fn new() -> Self { Self { cutoff: 1000.0, sample_rate: 48000.0, z1: 0.0, prev_in: 0.0 } }
}
impl Default for FilterHP { fn default() -> Self { Self::new() } }

impl DspNode for FilterHP {
    fn reset(&mut self, sr: f32) { self.sample_rate = sr; self.z1 = 0.0; self.prev_in = 0.0; }
    fn process(&mut self, _ctx: &ProcessCtx, ins: &[AudioBusRef], outs: &mut [AudioBusMut]) {
        if ins.is_empty() || outs.is_empty() { return; }
        let a = (-2.0 * PI * self.cutoff / self.sample_rate).exp();
        if let (Some(i), Some(o)) = (ins[0].channel(0), outs[0].channel(0)) {
            for (inp, outp) in i.iter().zip(o.iter_mut()) {
                self.z1 = a * (self.z1 + inp - self.prev_in);
                self.prev_in = *inp;
                *outp = self.z1;
            }
        }
    }
    fn set_param(&mut self, p: ParamIndex, v: ParamValue) { if p.0 == 0 { self.cutoff = v.as_float().clamp(20.0, 20000.0); } }
    fn handle_events(&mut self, _: &ProcessCtx, _: &mut EventQueue) {}
}
impl crate::NodeDescriptor for FilterHP {
    fn type_name(&self) -> &'static str { "FilterHP" }
    fn ports(&self) -> &'static [PortSpec] { PORTS }
    fn params(&self) -> &'static [ParamSpec] { PARAMS }
    fn display_name(&self) -> &'static str { "Highpass Filter" }
    fn category(&self) -> &'static str { "Filters" }
}
