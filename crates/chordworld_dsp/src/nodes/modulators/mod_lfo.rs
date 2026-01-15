//! Low-frequency oscillator (sine/saw/square/triangle/S&H)

use crate::{AudioBusMut, AudioBusRef, DspNode, ParamSpec, PortSpec, ProcessCtx};
use chordworld_core::{EventQueue, ParamIndex, ParamValue};
use std::f32::consts::TAU;

static PORTS: &[PortSpec] = &[PortSpec::audio_out("out", 1)];
static PARAMS: &[ParamSpec] = &[
    ParamSpec::new("rate", 1.0, 0.01, 50.0, "Hz"),
    ParamSpec::new("shape", 0.0, 0.0, 4.0, ""), // 0=sine,1=saw,2=square,3=tri,4=s&h
    ParamSpec::new("amount", 1.0, 0.0, 1.0, ""),
];

pub struct ModLFO {
    rate: f32, shape: u32, amount: f32,
    phase: f32, sh_value: f32, rng: u32, sample_rate: f32,
}

impl ModLFO {
    pub fn new() -> Self { Self { rate: 1.0, shape: 0, amount: 1.0, phase: 0.0, sh_value: 0.0, rng: 12345, sample_rate: 48000.0 } }
    fn rand(&mut self) -> f32 { self.rng = self.rng.wrapping_mul(1103515245).wrapping_add(12345); ((self.rng >> 16) & 0x7FFF) as f32 / 16383.5 - 1.0 }
    fn process_sample(&mut self) -> f32 {
        let _prev_phase = self.phase;
        self.phase += self.rate / self.sample_rate;
        if self.phase >= 1.0 { self.phase -= 1.0; if self.shape == 4 { self.sh_value = self.rand(); } }
        let raw = match self.shape {
            0 => (self.phase * TAU).sin(),
            1 => 2.0 * self.phase - 1.0,
            2 => if self.phase < 0.5 { 1.0 } else { -1.0 },
            3 => 2.0 * (2.0 * self.phase - 1.0).abs() - 1.0,
            _ => self.sh_value,
        };
        raw * self.amount
    }
}
impl Default for ModLFO { fn default() -> Self { Self::new() } }

impl DspNode for ModLFO {
    fn reset(&mut self, sr: f32) { self.sample_rate = sr; self.phase = 0.0; }
    fn process(&mut self, _: &ProcessCtx, _: &[AudioBusRef], outs: &mut [AudioBusMut]) {
        if let Some(ch) = outs.get_mut(0).and_then(|o| o.channel(0)) { for s in ch { *s = self.process_sample(); } }
    }
    fn set_param(&mut self, p: ParamIndex, v: ParamValue) {
        match p.0 { 0 => self.rate = v.as_float().clamp(0.01, 50.0), 1 => self.shape = v.as_float() as u32, 2 => self.amount = v.as_float().clamp(0.0, 1.0), _ => {} }
    }
    fn handle_events(&mut self, _: &ProcessCtx, _: &mut EventQueue) {}
}
impl crate::NodeDescriptor for ModLFO {
    fn type_name(&self) -> &'static str { "ModLFO" }
    fn ports(&self) -> &'static [PortSpec] { PORTS }
    fn params(&self) -> &'static [ParamSpec] { PARAMS }
    fn display_name(&self) -> &'static str { "LFO" }
    fn category(&self) -> &'static str { "Modulators" }
}
