//! 2-operator FM synthesis oscillator

use crate::{AudioBusMut, AudioBusRef, DspNode, ParamSpec, PortSpec, ProcessCtx};
use chordworld_core::{EventQueue, ParamIndex, ParamValue};
use std::f32::consts::TAU;

const PARAM_FREQ: u32 = 0;
const PARAM_AMP: u32 = 1;
const PARAM_RATIO: u32 = 2;
const PARAM_MOD_INDEX: u32 = 3;

static PORTS: &[PortSpec] = &[PortSpec::audio_out("out", 1)];

static PARAMS: &[ParamSpec] = &[
    ParamSpec::new("freq", 440.0, 20.0, 20000.0, "Hz"),
    ParamSpec::new("amp", 0.5, 0.0, 1.0, ""),
    ParamSpec::new("ratio", 2.0, 0.5, 16.0, ""), // Modulator/carrier ratio
    ParamSpec::new("mod_index", 1.0, 0.0, 10.0, ""), // Modulation depth
];

pub struct OscFM {
    carrier_phase: f32,
    mod_phase: f32,
    freq: f32,
    amp: f32,
    ratio: f32,
    mod_index: f32,
    sample_rate: f32,
}

impl OscFM {
    pub fn new() -> Self {
        Self {
            carrier_phase: 0.0,
            mod_phase: 0.0,
            freq: 440.0,
            amp: 0.5,
            ratio: 2.0,
            mod_index: 1.0,
            sample_rate: 48000.0,
        }
    }

    fn process_sample(&mut self) -> f32 {
        // Modulator
        let mod_freq = self.freq * self.ratio;
        let modulator = (self.mod_phase * TAU).sin();

        // Carrier with FM
        let fm = modulator * self.mod_index;
        let output = ((self.carrier_phase + fm) * TAU).sin() * self.amp;

        // Advance phases
        let carrier_inc = self.freq / self.sample_rate;
        let mod_inc = mod_freq / self.sample_rate;

        self.carrier_phase += carrier_inc;
        self.mod_phase += mod_inc;

        if self.carrier_phase >= 1.0 { self.carrier_phase -= 1.0; }
        if self.mod_phase >= 1.0 { self.mod_phase -= 1.0; }

        output
    }
}

impl Default for OscFM {
    fn default() -> Self { Self::new() }
}

impl DspNode for OscFM {
    fn reset(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;
        self.carrier_phase = 0.0;
        self.mod_phase = 0.0;
    }

    fn process(&mut self, _ctx: &ProcessCtx, _ins: &[AudioBusRef], outs: &mut [AudioBusMut]) {
        if outs.is_empty() { return; }
        let out = &mut outs[0];
        if let Some(channel) = out.channel(0) {
            for sample in channel {
                *sample = self.process_sample();
            }
        }
    }

    fn set_param(&mut self, param: ParamIndex, value: ParamValue) {
        match param.0 {
            PARAM_FREQ => self.freq = value.as_float().max(0.0),
            PARAM_AMP => self.amp = value.as_float().clamp(0.0, 1.0),
            PARAM_RATIO => self.ratio = value.as_float().clamp(0.5, 16.0),
            PARAM_MOD_INDEX => self.mod_index = value.as_float().clamp(0.0, 10.0),
            _ => {}
        }
    }

    fn handle_events(&mut self, _ctx: &ProcessCtx, _events: &mut EventQueue) {}
}

impl crate::NodeDescriptor for OscFM {
    fn type_name(&self) -> &'static str { "OscFM" }
    fn ports(&self) -> &'static [PortSpec] { PORTS }
    fn params(&self) -> &'static [ParamSpec] { PARAMS }
    fn display_name(&self) -> &'static str { "FM Oscillator" }
    fn category(&self) -> &'static str { "Oscillators" }
}
