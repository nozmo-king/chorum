//! Dynamics compressor

use crate::{AudioBusMut, AudioBusRef, DspNode, ParamSpec, PortSpec, ProcessCtx};
use chordworld_core::{EventQueue, ParamIndex, ParamValue};

static PORTS: &[PortSpec] = &[PortSpec::audio_in("in", 1), PortSpec::audio_out("out", 1)];
static PARAMS: &[ParamSpec] = &[
    ParamSpec::new("threshold", -20.0, -60.0, 0.0, "dB"),
    ParamSpec::new("ratio", 4.0, 1.0, 20.0, ""),
    ParamSpec::new("attack", 0.01, 0.001, 0.5, "s"),
    ParamSpec::new("release", 0.1, 0.01, 2.0, "s"),
    ParamSpec::new("makeup", 0.0, 0.0, 30.0, "dB"),
];

pub struct FxCompressor {
    threshold: f32, ratio: f32, attack: f32, release: f32, makeup: f32,
    envelope: f32, sample_rate: f32,
}

impl FxCompressor {
    pub fn new() -> Self {
        Self { threshold: -20.0, ratio: 4.0, attack: 0.01, release: 0.1, makeup: 0.0, envelope: 0.0, sample_rate: 48000.0 }
    }
    fn db_to_linear(db: f32) -> f32 { 10.0_f32.powf(db / 20.0) }
    fn linear_to_db(lin: f32) -> f32 { 20.0 * lin.abs().max(1e-10).log10() }
    fn process_sample(&mut self, input: f32) -> f32 {
        let input_db = Self::linear_to_db(input);
        let over_threshold = (input_db - self.threshold).max(0.0);
        let gain_reduction = over_threshold * (1.0 - 1.0 / self.ratio);
        let target_db = -gain_reduction;
        let target_lin = Self::db_to_linear(target_db);
        let coeff = if target_lin < self.envelope {
            (-1.0 / (self.attack * self.sample_rate)).exp()
        } else {
            (-1.0 / (self.release * self.sample_rate)).exp()
        };
        self.envelope = target_lin + coeff * (self.envelope - target_lin);
        let makeup_lin = Self::db_to_linear(self.makeup);
        input * self.envelope * makeup_lin
    }
}
impl Default for FxCompressor { fn default() -> Self { Self::new() } }

impl DspNode for FxCompressor {
    fn reset(&mut self, sr: f32) { self.sample_rate = sr; self.envelope = 1.0; }
    fn process(&mut self, _: &ProcessCtx, ins: &[AudioBusRef], outs: &mut [AudioBusMut]) {
        if ins.is_empty() || outs.is_empty() { return; }
        if let (Some(i), Some(o)) = (ins[0].channel(0), outs[0].channel(0)) {
            for (inp, outp) in i.iter().zip(o.iter_mut()) { *outp = self.process_sample(*inp); }
        }
    }
    fn set_param(&mut self, p: ParamIndex, v: ParamValue) {
        match p.0 {
            0 => self.threshold = v.as_float().clamp(-60.0, 0.0),
            1 => self.ratio = v.as_float().clamp(1.0, 20.0),
            2 => self.attack = v.as_float().clamp(0.001, 0.5),
            3 => self.release = v.as_float().clamp(0.01, 2.0),
            4 => self.makeup = v.as_float().clamp(0.0, 30.0),
            _ => {}
        }
    }
    fn handle_events(&mut self, _: &ProcessCtx, _: &mut EventQueue) {}
}
impl crate::NodeDescriptor for FxCompressor {
    fn type_name(&self) -> &'static str { "FxCompressor" }
    fn ports(&self) -> &'static [PortSpec] { PORTS }
    fn params(&self) -> &'static [ParamSpec] { PARAMS }
    fn display_name(&self) -> &'static str { "Compressor" }
    fn category(&self) -> &'static str { "Effects" }
}
