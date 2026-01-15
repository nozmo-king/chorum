//! Stereo delay with feedback

use crate::{AudioBusMut, AudioBusRef, DspNode, ParamSpec, PortSpec, ProcessCtx};
use chordworld_core::{EventQueue, ParamIndex, ParamValue};

const MAX_DELAY_SAMPLES: usize = 48000 * 2; // 2 seconds at 48kHz

static PORTS: &[PortSpec] = &[PortSpec::audio_in("in", 1), PortSpec::audio_out("out", 1)];
static PARAMS: &[ParamSpec] = &[
    ParamSpec::new("time", 0.25, 0.001, 2.0, "s"),
    ParamSpec::new("feedback", 0.4, 0.0, 0.95, ""),
    ParamSpec::new("mix", 0.5, 0.0, 1.0, ""),
];

pub struct FxDelay {
    time: f32, feedback: f32, mix: f32,
    buffer: Vec<f32>, write_pos: usize, sample_rate: f32,
}

impl FxDelay {
    pub fn new() -> Self {
        Self {
            time: 0.25, feedback: 0.4, mix: 0.5,
            buffer: vec![0.0; MAX_DELAY_SAMPLES], write_pos: 0, sample_rate: 48000.0,
        }
    }
    fn process_sample(&mut self, input: f32) -> f32 {
        let delay_samples = (self.time * self.sample_rate) as usize;
        let delay_samples = delay_samples.min(MAX_DELAY_SAMPLES - 1);
        let read_pos = (self.write_pos + MAX_DELAY_SAMPLES - delay_samples) % MAX_DELAY_SAMPLES;
        let delayed = self.buffer[read_pos];
        self.buffer[self.write_pos] = input + delayed * self.feedback;
        self.write_pos = (self.write_pos + 1) % MAX_DELAY_SAMPLES;
        input * (1.0 - self.mix) + delayed * self.mix
    }
}
impl Default for FxDelay { fn default() -> Self { Self::new() } }

impl DspNode for FxDelay {
    fn reset(&mut self, sr: f32) { self.sample_rate = sr; self.buffer.fill(0.0); self.write_pos = 0; }
    fn process(&mut self, _: &ProcessCtx, ins: &[AudioBusRef], outs: &mut [AudioBusMut]) {
        if ins.is_empty() || outs.is_empty() { return; }
        if let (Some(i), Some(o)) = (ins[0].channel(0), outs[0].channel(0)) {
            for (inp, outp) in i.iter().zip(o.iter_mut()) { *outp = self.process_sample(*inp); }
        }
    }
    fn set_param(&mut self, p: ParamIndex, v: ParamValue) {
        match p.0 { 0 => self.time = v.as_float().clamp(0.001, 2.0), 1 => self.feedback = v.as_float().clamp(0.0, 0.95), 2 => self.mix = v.as_float().clamp(0.0, 1.0), _ => {} }
    }
    fn handle_events(&mut self, _: &ProcessCtx, _: &mut EventQueue) {}
}
impl crate::NodeDescriptor for FxDelay {
    fn type_name(&self) -> &'static str { "FxDelay" }
    fn ports(&self) -> &'static [PortSpec] { PORTS }
    fn params(&self) -> &'static [ParamSpec] { PARAMS }
    fn display_name(&self) -> &'static str { "Delay" }
    fn category(&self) -> &'static str { "Effects" }
}
