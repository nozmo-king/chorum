//! Sawtooth wave oscillator - bright, rich harmonics

use crate::{AudioBusMut, AudioBusRef, DspNode, ParamSpec, PortSpec, ProcessCtx};
use chordworld_core::{EventQueue, ParamIndex, ParamValue};

const PARAM_FREQ: u32 = 0;
const PARAM_AMP: u32 = 1;

static PORTS: &[PortSpec] = &[PortSpec::audio_out("out", 1)];

static PARAMS: &[ParamSpec] = &[
    ParamSpec::new("freq", 440.0, 20.0, 20000.0, "Hz"),
    ParamSpec::new("amp", 0.5, 0.0, 1.0, ""),
];

pub struct OscSaw {
    phase: f32,
    freq: f32,
    amp: f32,
    sample_rate: f32,
}

impl OscSaw {
    pub fn new() -> Self {
        Self {
            phase: 0.0,
            freq: 440.0,
            amp: 0.5,
            sample_rate: 48000.0,
        }
    }

    fn process_sample(&mut self) -> f32 {
        // Sawtooth: linear ramp from -1 to 1
        let output = (2.0 * self.phase - 1.0) * self.amp;

        self.phase += self.freq / self.sample_rate;
        if self.phase >= 1.0 {
            self.phase -= 1.0;
        }

        output
    }
}

impl Default for OscSaw {
    fn default() -> Self {
        Self::new()
    }
}

impl DspNode for OscSaw {
    fn reset(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;
        self.phase = 0.0;
    }

    fn process(&mut self, _ctx: &ProcessCtx, _ins: &[AudioBusRef], outs: &mut [AudioBusMut]) {
        if outs.is_empty() {
            return;
        }

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
            _ => {}
        }
    }

    fn handle_events(&mut self, _ctx: &ProcessCtx, _events: &mut EventQueue) {}
}

impl crate::NodeDescriptor for OscSaw {
    fn type_name(&self) -> &'static str { "OscSaw" }
    fn ports(&self) -> &'static [PortSpec] { PORTS }
    fn params(&self) -> &'static [ParamSpec] { PARAMS }
    fn display_name(&self) -> &'static str { "Sawtooth Oscillator" }
    fn category(&self) -> &'static str { "Oscillators" }
}
