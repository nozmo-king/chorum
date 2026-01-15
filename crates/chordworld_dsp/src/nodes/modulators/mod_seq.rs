//! 8-step sequencer

use crate::{AudioBusMut, AudioBusRef, DspNode, ParamSpec, PortSpec, ProcessCtx};
use chordworld_core::{EventQueue, ParamIndex, ParamValue};

const STEPS: usize = 8;

static PORTS: &[PortSpec] = &[PortSpec::audio_out("out", 1)];
static PARAMS: &[ParamSpec] = &[
    ParamSpec::new("rate", 4.0, 0.1, 32.0, "Hz"),
    ParamSpec::new("steps", 8.0, 1.0, 8.0, ""),
    ParamSpec::new("step1", 0.5, 0.0, 1.0, ""),
    ParamSpec::new("step2", 0.6, 0.0, 1.0, ""),
    ParamSpec::new("step3", 0.7, 0.0, 1.0, ""),
    ParamSpec::new("step4", 0.8, 0.0, 1.0, ""),
    ParamSpec::new("step5", 0.6, 0.0, 1.0, ""),
    ParamSpec::new("step6", 0.4, 0.0, 1.0, ""),
    ParamSpec::new("step7", 0.3, 0.0, 1.0, ""),
    ParamSpec::new("step8", 0.2, 0.0, 1.0, ""),
];

pub struct ModSeq {
    rate: f32, num_steps: usize,
    values: [f32; STEPS],
    phase: f32, current_step: usize, sample_rate: f32,
}

impl ModSeq {
    pub fn new() -> Self {
        Self { rate: 4.0, num_steps: 8, values: [0.5, 0.6, 0.7, 0.8, 0.6, 0.4, 0.3, 0.2], phase: 0.0, current_step: 0, sample_rate: 48000.0 }
    }
    fn process_sample(&mut self) -> f32 {
        self.phase += self.rate / self.sample_rate;
        if self.phase >= 1.0 { self.phase -= 1.0; self.current_step = (self.current_step + 1) % self.num_steps; }
        self.values[self.current_step]
    }
}
impl Default for ModSeq { fn default() -> Self { Self::new() } }

impl DspNode for ModSeq {
    fn reset(&mut self, sr: f32) { self.sample_rate = sr; self.phase = 0.0; self.current_step = 0; }
    fn process(&mut self, _: &ProcessCtx, _: &[AudioBusRef], outs: &mut [AudioBusMut]) {
        if let Some(ch) = outs.get_mut(0).and_then(|o| o.channel(0)) { for s in ch { *s = self.process_sample(); } }
    }
    fn set_param(&mut self, p: ParamIndex, v: ParamValue) {
        match p.0 {
            0 => self.rate = v.as_float().clamp(0.1, 32.0),
            1 => self.num_steps = (v.as_float() as usize).clamp(1, STEPS),
            n if n >= 2 && n < 10 => self.values[n as usize - 2] = v.as_float().clamp(0.0, 1.0),
            _ => {}
        }
    }
    fn handle_events(&mut self, _: &ProcessCtx, _: &mut EventQueue) {}
}
impl crate::NodeDescriptor for ModSeq {
    fn type_name(&self) -> &'static str { "ModSeq" }
    fn ports(&self) -> &'static [PortSpec] { PORTS }
    fn params(&self) -> &'static [ParamSpec] { PARAMS }
    fn display_name(&self) -> &'static str { "Step Sequencer" }
    fn category(&self) -> &'static str { "Modulators" }
}
