//! Voltage Controlled Amplifier

use crate::{AudioBusMut, AudioBusRef, DspNode, ParamSpec, PortSpec, ProcessCtx};
use chordworld_core::{EventQueue, ParamIndex, ParamValue};

static PORTS: &[PortSpec] = &[
    PortSpec::audio_in("in", 1),
    PortSpec::audio_in("cv", 1),
    PortSpec::audio_out("out", 1),
];
static PARAMS: &[ParamSpec] = &[
    ParamSpec::new("gain", 1.0, 0.0, 2.0, ""),
    ParamSpec::new("cv_amount", 1.0, 0.0, 1.0, ""),
    ParamSpec::new("linear", 0.0, 0.0, 1.0, ""), // 0=exponential, 1=linear
];

pub struct UtilVCA {
    gain: f32,
    cv_amount: f32,
    linear: bool,
}

impl UtilVCA {
    pub fn new() -> Self { Self { gain: 1.0, cv_amount: 1.0, linear: false } }
}
impl Default for UtilVCA { fn default() -> Self { Self::new() } }

impl DspNode for UtilVCA {
    fn reset(&mut self, _: f32) {}
    fn process(&mut self, _: &ProcessCtx, ins: &[AudioBusRef], outs: &mut [AudioBusMut]) {
        if ins.is_empty() || outs.is_empty() { return; }
        let in_ch = ins[0].channel(0);
        let cv_ch = ins.get(1).and_then(|b| b.channel(0));
        let out_ch = outs[0].channel(0);
        if let (Some(i), Some(o)) = (in_ch, out_ch) {
            for (idx, (inp, outp)) in i.iter().zip(o.iter_mut()).enumerate() {
                let cv = cv_ch.map(|c| c.get(idx).copied().unwrap_or(1.0)).unwrap_or(1.0);
                let cv_scaled = cv * self.cv_amount + (1.0 - self.cv_amount);
                let effective_gain = if self.linear {
                    self.gain * cv_scaled.max(0.0)
                } else {
                    self.gain * (cv_scaled.max(0.0).powf(2.0))
                };
                *outp = inp * effective_gain;
            }
        }
    }
    fn set_param(&mut self, p: ParamIndex, v: ParamValue) {
        match p.0 {
            0 => self.gain = v.as_float().clamp(0.0, 2.0),
            1 => self.cv_amount = v.as_float().clamp(0.0, 1.0),
            2 => self.linear = v.as_float() > 0.5,
            _ => {}
        }
    }
    fn handle_events(&mut self, _: &ProcessCtx, _: &mut EventQueue) {}
}
impl crate::NodeDescriptor for UtilVCA {
    fn type_name(&self) -> &'static str { "UtilVCA" }
    fn ports(&self) -> &'static [PortSpec] { PORTS }
    fn params(&self) -> &'static [ParamSpec] { PARAMS }
    fn display_name(&self) -> &'static str { "VCA" }
    fn category(&self) -> &'static str { "Utilities" }
}
