//! Waveshaping distortion

use crate::{AudioBusMut, AudioBusRef, DspNode, ParamSpec, PortSpec, ProcessCtx};
use chordworld_core::{EventQueue, ParamIndex, ParamValue};

static PORTS: &[PortSpec] = &[PortSpec::audio_in("in", 1), PortSpec::audio_out("out", 1)];
static PARAMS: &[ParamSpec] = &[
    ParamSpec::new("drive", 1.0, 0.1, 100.0, ""),
    ParamSpec::new("tone", 0.5, 0.0, 1.0, ""),
    ParamSpec::new("mix", 1.0, 0.0, 1.0, ""),
];

pub struct FxDistortion { drive: f32, tone: f32, mix: f32, lp_state: f32 }

impl FxDistortion {
    pub fn new() -> Self { Self { drive: 1.0, tone: 0.5, mix: 1.0, lp_state: 0.0 } }
    fn process_sample(&mut self, input: f32) -> f32 {
        let driven = input * self.drive;
        let shaped = driven.tanh(); // Soft clipping
        // Simple tone control (lowpass)
        self.lp_state = self.lp_state + self.tone * (shaped - self.lp_state);
        input * (1.0 - self.mix) + self.lp_state * self.mix
    }
}
impl Default for FxDistortion { fn default() -> Self { Self::new() } }

impl DspNode for FxDistortion {
    fn reset(&mut self, _: f32) { self.lp_state = 0.0; }
    fn process(&mut self, _: &ProcessCtx, ins: &[AudioBusRef], outs: &mut [AudioBusMut]) {
        if ins.is_empty() || outs.is_empty() { return; }
        if let (Some(i), Some(o)) = (ins[0].channel(0), outs[0].channel(0)) {
            for (inp, outp) in i.iter().zip(o.iter_mut()) { *outp = self.process_sample(*inp); }
        }
    }
    fn set_param(&mut self, p: ParamIndex, v: ParamValue) {
        match p.0 { 0 => self.drive = v.as_float().clamp(0.1, 100.0), 1 => self.tone = v.as_float().clamp(0.0, 1.0), 2 => self.mix = v.as_float().clamp(0.0, 1.0), _ => {} }
    }
    fn handle_events(&mut self, _: &ProcessCtx, _: &mut EventQueue) {}
}
impl crate::NodeDescriptor for FxDistortion {
    fn type_name(&self) -> &'static str { "FxDistortion" }
    fn ports(&self) -> &'static [PortSpec] { PORTS }
    fn params(&self) -> &'static [ParamSpec] { PARAMS }
    fn display_name(&self) -> &'static str { "Distortion" }
    fn category(&self) -> &'static str { "Effects" }
}
