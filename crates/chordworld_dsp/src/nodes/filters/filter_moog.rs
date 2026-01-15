//! Moog ladder filter - classic analog sound

use crate::{AudioBusMut, AudioBusRef, DspNode, ParamSpec, PortSpec, ProcessCtx};
use chordworld_core::{EventQueue, ParamIndex, ParamValue};

const PARAM_CUTOFF: u32 = 0;
const PARAM_RESONANCE: u32 = 1;

static PORTS: &[PortSpec] = &[
    PortSpec::audio_in("in", 1),
    PortSpec::audio_out("out", 1),
];

static PARAMS: &[ParamSpec] = &[
    ParamSpec::new("cutoff", 1000.0, 20.0, 20000.0, "Hz"),
    ParamSpec::new("resonance", 0.0, 0.0, 4.0, ""), // 4 = self-oscillation
];

pub struct FilterMoog {
    cutoff: f32,
    resonance: f32,
    sample_rate: f32,
    // 4-pole state
    stage: [f32; 4],
    delay: [f32; 4],
}

impl FilterMoog {
    pub fn new() -> Self {
        Self {
            cutoff: 1000.0,
            resonance: 0.0,
            sample_rate: 48000.0,
            stage: [0.0; 4],
            delay: [0.0; 4],
        }
    }

    fn process_sample(&mut self, input: f32) -> f32 {
        let f = 2.0 * self.cutoff / self.sample_rate;
        let f = f.min(0.99);
        let k = 3.6 * f - 1.6 * f * f - 1.0;
        let p = (k + 1.0) * 0.5;
        let scale = (-3.0_f32 / 2.0).exp();
        let r = self.resonance * scale;

        // Feedback
        let x = input - r * self.stage[3];

        // Four cascaded one-pole filters
        self.stage[0] = x * p + self.delay[0] * p - k * self.stage[0];
        self.delay[0] = x;
        self.stage[1] = self.stage[0] * p + self.delay[1] * p - k * self.stage[1];
        self.delay[1] = self.stage[0];
        self.stage[2] = self.stage[1] * p + self.delay[2] * p - k * self.stage[2];
        self.delay[2] = self.stage[1];
        self.stage[3] = self.stage[2] * p + self.delay[3] * p - k * self.stage[3];
        self.delay[3] = self.stage[2];

        self.stage[3]
    }
}

impl Default for FilterMoog {
    fn default() -> Self { Self::new() }
}

impl DspNode for FilterMoog {
    fn reset(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;
        self.stage = [0.0; 4];
        self.delay = [0.0; 4];
    }

    fn process(&mut self, _ctx: &ProcessCtx, ins: &[AudioBusRef], outs: &mut [AudioBusMut]) {
        if ins.is_empty() || outs.is_empty() { return; }
        if let (Some(in_ch), Some(out_ch)) = (ins[0].channel(0), outs[0].channel(0)) {
            for (inp, outp) in in_ch.iter().zip(out_ch.iter_mut()) {
                *outp = self.process_sample(*inp);
            }
        }
    }

    fn set_param(&mut self, param: ParamIndex, value: ParamValue) {
        match param.0 {
            PARAM_CUTOFF => self.cutoff = value.as_float().clamp(20.0, 20000.0),
            PARAM_RESONANCE => self.resonance = value.as_float().clamp(0.0, 4.0),
            _ => {}
        }
    }

    fn handle_events(&mut self, _ctx: &ProcessCtx, _events: &mut EventQueue) {}
}

impl crate::NodeDescriptor for FilterMoog {
    fn type_name(&self) -> &'static str { "FilterMoog" }
    fn ports(&self) -> &'static [PortSpec] { PORTS }
    fn params(&self) -> &'static [ParamSpec] { PARAMS }
    fn display_name(&self) -> &'static str { "Moog Ladder Filter" }
    fn category(&self) -> &'static str { "Filters" }
}
