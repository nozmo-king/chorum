//! Sample and hold

use crate::{AudioBusMut, AudioBusRef, DspNode, ParamSpec, PortSpec, ProcessCtx};
use chordworld_core::{EventQueue, ParamIndex, ParamValue};

static PORTS: &[PortSpec] = &[PortSpec::audio_in("in", 1), PortSpec::audio_out("out", 1)];
static PARAMS: &[ParamSpec] = &[
    ParamSpec::new("rate", 4.0, 0.1, 100.0, "Hz"),
    ParamSpec::new("smooth", 0.0, 0.0, 1.0, ""),
];

pub struct ModSampleHold {
    rate: f32, smooth: f32,
    phase: f32, held: f32, smoothed: f32, sample_rate: f32,
}

impl ModSampleHold {
    pub fn new() -> Self { Self { rate: 4.0, smooth: 0.0, phase: 0.0, held: 0.0, smoothed: 0.0, sample_rate: 48000.0 } }
}
impl Default for ModSampleHold { fn default() -> Self { Self::new() } }

impl DspNode for ModSampleHold {
    fn reset(&mut self, sr: f32) { self.sample_rate = sr; self.phase = 0.0; self.held = 0.0; self.smoothed = 0.0; }
    fn process(&mut self, _: &ProcessCtx, ins: &[AudioBusRef], outs: &mut [AudioBusMut]) {
        if ins.is_empty() || outs.is_empty() { return; }
        if let (Some(i), Some(o)) = (ins[0].channel(0), outs[0].channel(0)) {
            for (inp, outp) in i.iter().zip(o.iter_mut()) {
                self.phase += self.rate / self.sample_rate;
                if self.phase >= 1.0 { self.phase -= 1.0; self.held = *inp; }
                self.smoothed = self.smoothed + (1.0 - self.smooth) * (self.held - self.smoothed);
                *outp = self.smoothed;
            }
        }
    }
    fn set_param(&mut self, p: ParamIndex, v: ParamValue) {
        match p.0 { 0 => self.rate = v.as_float().clamp(0.1, 100.0), 1 => self.smooth = v.as_float().clamp(0.0, 1.0), _ => {} }
    }
    fn handle_events(&mut self, _: &ProcessCtx, _: &mut EventQueue) {}
}
impl crate::NodeDescriptor for ModSampleHold {
    fn type_name(&self) -> &'static str { "ModSampleHold" }
    fn ports(&self) -> &'static [PortSpec] { PORTS }
    fn params(&self) -> &'static [ParamSpec] { PARAMS }
    fn display_name(&self) -> &'static str { "Sample & Hold" }
    fn category(&self) -> &'static str { "Modulators" }
}
