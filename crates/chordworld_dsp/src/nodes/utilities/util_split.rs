//! Signal splitter (1 to 4)

use crate::{AudioBusMut, AudioBusRef, DspNode, ParamSpec, PortSpec, ProcessCtx};
use chordworld_core::{EventQueue, ParamIndex, ParamValue};

static PORTS: &[PortSpec] = &[
    PortSpec::audio_in("in", 1),
    PortSpec::audio_out("out1", 1),
    PortSpec::audio_out("out2", 1),
    PortSpec::audio_out("out3", 1),
    PortSpec::audio_out("out4", 1),
];
static PARAMS: &[ParamSpec] = &[
    ParamSpec::new("gain", 1.0, 0.0, 2.0, ""),
];

pub struct UtilSplit {
    gain: f32,
}

impl UtilSplit {
    pub fn new() -> Self { Self { gain: 1.0 } }
}
impl Default for UtilSplit { fn default() -> Self { Self::new() } }

impl DspNode for UtilSplit {
    fn reset(&mut self, _: f32) {}
    fn process(&mut self, _: &ProcessCtx, ins: &[AudioBusRef], outs: &mut [AudioBusMut]) {
        if ins.is_empty() { return; }
        if let Some(in_ch) = ins[0].channel(0) {
            for out_bus in outs.iter_mut() {
                if let Some(out_ch) = out_bus.channel(0) {
                    for (inp, outp) in in_ch.iter().zip(out_ch.iter_mut()) {
                        *outp = inp * self.gain;
                    }
                }
            }
        }
    }
    fn set_param(&mut self, p: ParamIndex, v: ParamValue) {
        if p.0 == 0 { self.gain = v.as_float().clamp(0.0, 2.0); }
    }
    fn handle_events(&mut self, _: &ProcessCtx, _: &mut EventQueue) {}
}
impl crate::NodeDescriptor for UtilSplit {
    fn type_name(&self) -> &'static str { "UtilSplit" }
    fn ports(&self) -> &'static [PortSpec] { PORTS }
    fn params(&self) -> &'static [ParamSpec] { PARAMS }
    fn display_name(&self) -> &'static str { "Split" }
    fn category(&self) -> &'static str { "Utilities" }
}
