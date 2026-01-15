//! Signal merger (mono to stereo)

use crate::{AudioBusMut, AudioBusRef, DspNode, ParamSpec, PortSpec, ProcessCtx};
use chordworld_core::{EventQueue, ParamIndex, ParamValue};

static PORTS: &[PortSpec] = &[
    PortSpec::audio_in("left", 1),
    PortSpec::audio_in("right", 1),
    PortSpec::audio_out("out", 2),
];
static PARAMS: &[ParamSpec] = &[
    ParamSpec::new("balance", 0.5, 0.0, 1.0, ""), // 0=left, 0.5=center, 1=right
    ParamSpec::new("gain", 1.0, 0.0, 2.0, ""),
];

pub struct UtilMerge {
    balance: f32,
    gain: f32,
}

impl UtilMerge {
    pub fn new() -> Self { Self { balance: 0.5, gain: 1.0 } }
}
impl Default for UtilMerge { fn default() -> Self { Self::new() } }

impl DspNode for UtilMerge {
    fn reset(&mut self, _: f32) {}
    fn process(&mut self, _: &ProcessCtx, ins: &[AudioBusRef], outs: &mut [AudioBusMut]) {
        if outs.is_empty() { return; }
        let left_ch = ins.get(0).and_then(|b| b.channel(0));
        let right_ch = ins.get(1).and_then(|b| b.channel(0));

        let left_gain = (1.0 - self.balance) * 2.0 * self.gain;
        let right_gain = self.balance * 2.0 * self.gain;

        // Write left channel first
        if let Some(out_bus) = outs.get_mut(0) {
            if let Some(ol) = out_bus.channel(0) {
                for (i, outp) in ol.iter_mut().enumerate() {
                    let l = left_ch.and_then(|c| c.get(i)).copied().unwrap_or(0.0);
                    *outp = l * left_gain;
                }
            }
        }
        // Then write right channel
        if let Some(out_bus) = outs.get_mut(0) {
            if let Some(or) = out_bus.channel(1) {
                for (i, outp) in or.iter_mut().enumerate() {
                    let r = right_ch.and_then(|c| c.get(i)).copied().unwrap_or(0.0);
                    *outp = r * right_gain;
                }
            }
        }
    }
    fn set_param(&mut self, p: ParamIndex, v: ParamValue) {
        match p.0 {
            0 => self.balance = v.as_float().clamp(0.0, 1.0),
            1 => self.gain = v.as_float().clamp(0.0, 2.0),
            _ => {}
        }
    }
    fn handle_events(&mut self, _: &ProcessCtx, _: &mut EventQueue) {}
}
impl crate::NodeDescriptor for UtilMerge {
    fn type_name(&self) -> &'static str { "UtilMerge" }
    fn ports(&self) -> &'static [PortSpec] { PORTS }
    fn params(&self) -> &'static [ParamSpec] { PARAMS }
    fn display_name(&self) -> &'static str { "Merge" }
    fn category(&self) -> &'static str { "Utilities" }
}
