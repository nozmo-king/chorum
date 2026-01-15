//! 4-channel audio mixer

use crate::{AudioBusMut, AudioBusRef, DspNode, ParamSpec, PortSpec, ProcessCtx};
use chordworld_core::{EventQueue, ParamIndex, ParamValue};

static PORTS: &[PortSpec] = &[
    PortSpec::audio_in("in1", 1),
    PortSpec::audio_in("in2", 1),
    PortSpec::audio_in("in3", 1),
    PortSpec::audio_in("in4", 1),
    PortSpec::audio_out("out", 1),
];
static PARAMS: &[ParamSpec] = &[
    ParamSpec::new("gain1", 1.0, 0.0, 2.0, ""),
    ParamSpec::new("gain2", 1.0, 0.0, 2.0, ""),
    ParamSpec::new("gain3", 1.0, 0.0, 2.0, ""),
    ParamSpec::new("gain4", 1.0, 0.0, 2.0, ""),
    ParamSpec::new("master", 1.0, 0.0, 2.0, ""),
];

pub struct UtilMixer {
    gains: [f32; 4],
    master: f32,
}

impl UtilMixer {
    pub fn new() -> Self { Self { gains: [1.0; 4], master: 1.0 } }
}
impl Default for UtilMixer { fn default() -> Self { Self::new() } }

impl DspNode for UtilMixer {
    fn reset(&mut self, _: f32) {}
    fn process(&mut self, _: &ProcessCtx, ins: &[AudioBusRef], outs: &mut [AudioBusMut]) {
        if let Some(out_ch) = outs.get_mut(0).and_then(|o| o.channel(0)) {
            for s in out_ch.iter_mut() { *s = 0.0; }
            for (i, gain) in self.gains.iter().enumerate() {
                if let Some(in_ch) = ins.get(i).and_then(|b| b.channel(0)) {
                    for (j, sample) in in_ch.iter().enumerate() {
                        if j < out_ch.len() { out_ch[j] += sample * gain * self.master; }
                    }
                }
            }
        }
    }
    fn set_param(&mut self, p: ParamIndex, v: ParamValue) {
        match p.0 {
            0..=3 => self.gains[p.0 as usize] = v.as_float().clamp(0.0, 2.0),
            4 => self.master = v.as_float().clamp(0.0, 2.0),
            _ => {}
        }
    }
    fn handle_events(&mut self, _: &ProcessCtx, _: &mut EventQueue) {}
}
impl crate::NodeDescriptor for UtilMixer {
    fn type_name(&self) -> &'static str { "UtilMixer" }
    fn ports(&self) -> &'static [PortSpec] { PORTS }
    fn params(&self) -> &'static [ParamSpec] { PARAMS }
    fn display_name(&self) -> &'static str { "Mixer" }
    fn category(&self) -> &'static str { "Utilities" }
}
