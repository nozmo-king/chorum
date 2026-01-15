//! Slew rate limiter (portamento/glide)

use crate::{AudioBusMut, AudioBusRef, DspNode, ParamSpec, PortSpec, ProcessCtx};
use chordworld_core::{EventQueue, ParamIndex, ParamValue};

static PORTS: &[PortSpec] = &[PortSpec::audio_in("in", 1), PortSpec::audio_out("out", 1)];
static PARAMS: &[ParamSpec] = &[
    ParamSpec::new("rise", 0.1, 0.001, 10.0, "s"),
    ParamSpec::new("fall", 0.1, 0.001, 10.0, "s"),
];

pub struct ModSlew {
    rise: f32, fall: f32,
    current: f32, sample_rate: f32,
}

impl ModSlew {
    pub fn new() -> Self { Self { rise: 0.1, fall: 0.1, current: 0.0, sample_rate: 48000.0 } }
}
impl Default for ModSlew { fn default() -> Self { Self::new() } }

impl DspNode for ModSlew {
    fn reset(&mut self, sr: f32) { self.sample_rate = sr; self.current = 0.0; }
    fn process(&mut self, _: &ProcessCtx, ins: &[AudioBusRef], outs: &mut [AudioBusMut]) {
        if ins.is_empty() || outs.is_empty() { return; }
        if let (Some(i), Some(o)) = (ins[0].channel(0), outs[0].channel(0)) {
            for (inp, outp) in i.iter().zip(o.iter_mut()) {
                let target = *inp;
                let rate = if target > self.current { 1.0 / (self.rise * self.sample_rate) } else { 1.0 / (self.fall * self.sample_rate) };
                let diff = target - self.current;
                if diff.abs() < rate { self.current = target; } else { self.current += rate * diff.signum(); }
                *outp = self.current;
            }
        }
    }
    fn set_param(&mut self, p: ParamIndex, v: ParamValue) {
        match p.0 { 0 => self.rise = v.as_float().clamp(0.001, 10.0), 1 => self.fall = v.as_float().clamp(0.001, 10.0), _ => {} }
    }
    fn handle_events(&mut self, _: &ProcessCtx, _: &mut EventQueue) {}
}
impl crate::NodeDescriptor for ModSlew {
    fn type_name(&self) -> &'static str { "ModSlew" }
    fn ports(&self) -> &'static [PortSpec] { PORTS }
    fn params(&self) -> &'static [ParamSpec] { PARAMS }
    fn display_name(&self) -> &'static str { "Slew Limiter" }
    fn category(&self) -> &'static str { "Modulators" }
}
