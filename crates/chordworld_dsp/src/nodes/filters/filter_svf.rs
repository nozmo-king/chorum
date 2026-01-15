//! State Variable Filter - LP/HP/BP/Notch modes

use crate::{AudioBusMut, AudioBusRef, DspNode, ParamSpec, PortSpec, ProcessCtx};
use chordworld_core::{EventQueue, ParamIndex, ParamValue};
use std::f32::consts::PI;

const PARAM_CUTOFF: u32 = 0;
const PARAM_RESONANCE: u32 = 1;
const PARAM_MODE: u32 = 2;

static PORTS: &[PortSpec] = &[
    PortSpec::audio_in("in", 1),
    PortSpec::audio_out("out", 1),
];

static PARAMS: &[ParamSpec] = &[
    ParamSpec::new("cutoff", 1000.0, 20.0, 20000.0, "Hz"),
    ParamSpec::new("resonance", 0.5, 0.0, 1.0, ""),
    ParamSpec::new("mode", 0.0, 0.0, 3.0, ""), // 0=LP, 1=HP, 2=BP, 3=Notch
];

pub struct FilterSVF {
    cutoff: f32,
    resonance: f32,
    mode: u32,
    sample_rate: f32,
    // State
    ic1eq: f32,
    ic2eq: f32,
}

impl FilterSVF {
    pub fn new() -> Self {
        Self {
            cutoff: 1000.0,
            resonance: 0.5,
            mode: 0,
            sample_rate: 48000.0,
            ic1eq: 0.0,
            ic2eq: 0.0,
        }
    }

    fn process_sample(&mut self, input: f32) -> f32 {
        let g = (PI * self.cutoff / self.sample_rate).tan();
        let k = 2.0 - 2.0 * self.resonance;

        let v0 = input;
        let v3 = v0 - self.ic2eq;
        let v1 = self.ic1eq + g * v3 / (1.0 + g * (g + k));
        let v2 = self.ic2eq + g * v1;

        self.ic1eq = 2.0 * v1 - self.ic1eq;
        self.ic2eq = 2.0 * v2 - self.ic2eq;

        match self.mode {
            0 => v2,                    // Lowpass
            1 => v0 - k * v1 - v2,      // Highpass
            2 => v1,                    // Bandpass
            _ => v0 - k * v1,           // Notch
        }
    }
}

impl Default for FilterSVF {
    fn default() -> Self { Self::new() }
}

impl DspNode for FilterSVF {
    fn reset(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;
        self.ic1eq = 0.0;
        self.ic2eq = 0.0;
    }

    fn process(&mut self, _ctx: &ProcessCtx, ins: &[AudioBusRef], outs: &mut [AudioBusMut]) {
        if ins.is_empty() || outs.is_empty() { return; }
        let input = &ins[0];
        let output = &mut outs[0];

        if let (Some(in_ch), Some(out_ch)) = (input.channel(0), output.channel(0)) {
            for (inp, outp) in in_ch.iter().zip(out_ch.iter_mut()) {
                *outp = self.process_sample(*inp);
            }
        }
    }

    fn set_param(&mut self, param: ParamIndex, value: ParamValue) {
        match param.0 {
            PARAM_CUTOFF => self.cutoff = value.as_float().clamp(20.0, 20000.0),
            PARAM_RESONANCE => self.resonance = value.as_float().clamp(0.0, 1.0),
            PARAM_MODE => self.mode = value.as_float() as u32,
            _ => {}
        }
    }

    fn handle_events(&mut self, _ctx: &ProcessCtx, _events: &mut EventQueue) {}
}

impl crate::NodeDescriptor for FilterSVF {
    fn type_name(&self) -> &'static str { "FilterSVF" }
    fn ports(&self) -> &'static [PortSpec] { PORTS }
    fn params(&self) -> &'static [ParamSpec] { PARAMS }
    fn display_name(&self) -> &'static str { "State Variable Filter" }
    fn category(&self) -> &'static str { "Filters" }
}
