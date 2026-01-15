//! Algorithmic reverb (Freeverb-style)

use crate::{AudioBusMut, AudioBusRef, DspNode, ParamSpec, PortSpec, ProcessCtx};
use chordworld_core::{EventQueue, ParamIndex, ParamValue};

const NUM_COMBS: usize = 8;
const NUM_ALLPASS: usize = 4;
const COMB_LENGTHS: [usize; NUM_COMBS] = [1116, 1188, 1277, 1356, 1422, 1491, 1557, 1617];
const ALLPASS_LENGTHS: [usize; NUM_ALLPASS] = [556, 441, 341, 225];

static PORTS: &[PortSpec] = &[PortSpec::audio_in("in", 1), PortSpec::audio_out("out", 1)];
static PARAMS: &[ParamSpec] = &[
    ParamSpec::new("room", 0.5, 0.0, 1.0, ""),
    ParamSpec::new("damp", 0.5, 0.0, 1.0, ""),
    ParamSpec::new("mix", 0.3, 0.0, 1.0, ""),
];

struct CombFilter { buffer: Vec<f32>, pos: usize, feedback: f32, damp: f32, damp_state: f32 }
impl CombFilter {
    fn new(size: usize) -> Self { Self { buffer: vec![0.0; size], pos: 0, feedback: 0.5, damp: 0.5, damp_state: 0.0 } }
    fn process(&mut self, input: f32) -> f32 {
        let output = self.buffer[self.pos];
        self.damp_state = output * (1.0 - self.damp) + self.damp_state * self.damp;
        self.buffer[self.pos] = input + self.damp_state * self.feedback;
        self.pos = (self.pos + 1) % self.buffer.len();
        output
    }
}

struct AllpassFilter { buffer: Vec<f32>, pos: usize }
impl AllpassFilter {
    fn new(size: usize) -> Self { Self { buffer: vec![0.0; size], pos: 0 } }
    fn process(&mut self, input: f32) -> f32 {
        let buffered = self.buffer[self.pos];
        let output = -input + buffered;
        self.buffer[self.pos] = input + buffered * 0.5;
        self.pos = (self.pos + 1) % self.buffer.len();
        output
    }
}

pub struct FxReverb {
    room: f32, damp: f32, mix: f32,
    combs: Vec<CombFilter>, allpasses: Vec<AllpassFilter>,
}

impl FxReverb {
    pub fn new() -> Self {
        Self {
            room: 0.5, damp: 0.5, mix: 0.3,
            combs: COMB_LENGTHS.iter().map(|&s| CombFilter::new(s)).collect(),
            allpasses: ALLPASS_LENGTHS.iter().map(|&s| AllpassFilter::new(s)).collect(),
        }
    }
    fn process_sample(&mut self, input: f32) -> f32 {
        let mut out = 0.0;
        for comb in &mut self.combs {
            comb.feedback = self.room * 0.28 + 0.7;
            comb.damp = self.damp;
            out += comb.process(input);
        }
        out /= NUM_COMBS as f32;
        for ap in &mut self.allpasses { out = ap.process(out); }
        input * (1.0 - self.mix) + out * self.mix
    }
}
impl Default for FxReverb { fn default() -> Self { Self::new() } }

impl DspNode for FxReverb {
    fn reset(&mut self, _: f32) {
        for c in &mut self.combs { c.buffer.fill(0.0); c.pos = 0; c.damp_state = 0.0; }
        for a in &mut self.allpasses { a.buffer.fill(0.0); a.pos = 0; }
    }
    fn process(&mut self, _: &ProcessCtx, ins: &[AudioBusRef], outs: &mut [AudioBusMut]) {
        if ins.is_empty() || outs.is_empty() { return; }
        if let (Some(i), Some(o)) = (ins[0].channel(0), outs[0].channel(0)) {
            for (inp, outp) in i.iter().zip(o.iter_mut()) { *outp = self.process_sample(*inp); }
        }
    }
    fn set_param(&mut self, p: ParamIndex, v: ParamValue) {
        match p.0 { 0 => self.room = v.as_float().clamp(0.0, 1.0), 1 => self.damp = v.as_float().clamp(0.0, 1.0), 2 => self.mix = v.as_float().clamp(0.0, 1.0), _ => {} }
    }
    fn handle_events(&mut self, _: &ProcessCtx, _: &mut EventQueue) {}
}
impl crate::NodeDescriptor for FxReverb {
    fn type_name(&self) -> &'static str { "FxReverb" }
    fn ports(&self) -> &'static [PortSpec] { PORTS }
    fn params(&self) -> &'static [ParamSpec] { PARAMS }
    fn display_name(&self) -> &'static str { "Reverb" }
    fn category(&self) -> &'static str { "Effects" }
}
