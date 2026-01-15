//! Noise generator - white/pink/brown noise

use crate::{AudioBusMut, AudioBusRef, DspNode, ParamSpec, PortSpec, ProcessCtx};
use chordworld_core::{EventQueue, ParamIndex, ParamValue};

const PARAM_AMP: u32 = 0;
const PARAM_TYPE: u32 = 1;

static PORTS: &[PortSpec] = &[PortSpec::audio_out("out", 1)];

static PARAMS: &[ParamSpec] = &[
    ParamSpec::new("amp", 0.5, 0.0, 1.0, ""),
    ParamSpec::new("type", 0.0, 0.0, 2.0, ""), // 0=white, 1=pink, 2=brown
];

pub struct OscNoise {
    amp: f32,
    noise_type: u32,
    // Pink noise state (Paul Kellet's approximation)
    b0: f32, b1: f32, b2: f32, b3: f32, b4: f32, b5: f32, b6: f32,
    // Brown noise state
    brown: f32,
    // Simple LCG RNG state
    rng_state: u32,
}

impl OscNoise {
    pub fn new() -> Self {
        Self {
            amp: 0.5,
            noise_type: 0,
            b0: 0.0, b1: 0.0, b2: 0.0, b3: 0.0, b4: 0.0, b5: 0.0, b6: 0.0,
            brown: 0.0,
            rng_state: 12345,
        }
    }

    fn rand(&mut self) -> f32 {
        // Simple LCG
        self.rng_state = self.rng_state.wrapping_mul(1103515245).wrapping_add(12345);
        let val = ((self.rng_state >> 16) & 0x7FFF) as f32 / 16383.5 - 1.0;
        val
    }

    fn white(&mut self) -> f32 {
        self.rand() * self.amp
    }

    fn pink(&mut self) -> f32 {
        let white = self.rand();
        self.b0 = 0.99886 * self.b0 + white * 0.0555179;
        self.b1 = 0.99332 * self.b1 + white * 0.0750759;
        self.b2 = 0.96900 * self.b2 + white * 0.1538520;
        self.b3 = 0.86650 * self.b3 + white * 0.3104856;
        self.b4 = 0.55000 * self.b4 + white * 0.5329522;
        self.b5 = -0.7616 * self.b5 - white * 0.0168980;
        let pink = self.b0 + self.b1 + self.b2 + self.b3 + self.b4 + self.b5 + self.b6 + white * 0.5362;
        self.b6 = white * 0.115926;
        pink * 0.11 * self.amp
    }

    fn brown(&mut self) -> f32 {
        let white = self.rand();
        self.brown = (self.brown + (0.02 * white)) / 1.02;
        self.brown * 3.5 * self.amp
    }

    fn process_sample(&mut self) -> f32 {
        match self.noise_type {
            0 => self.white(),
            1 => self.pink(),
            _ => self.brown(),
        }
    }
}

impl Default for OscNoise {
    fn default() -> Self { Self::new() }
}

impl DspNode for OscNoise {
    fn reset(&mut self, _sample_rate: f32) {
        self.b0 = 0.0; self.b1 = 0.0; self.b2 = 0.0;
        self.b3 = 0.0; self.b4 = 0.0; self.b5 = 0.0; self.b6 = 0.0;
        self.brown = 0.0;
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
            PARAM_AMP => self.amp = value.as_float().clamp(0.0, 1.0),
            PARAM_TYPE => self.noise_type = value.as_float() as u32,
            _ => {}
        }
    }

    fn handle_events(&mut self, _ctx: &ProcessCtx, _events: &mut EventQueue) {}
}

impl crate::NodeDescriptor for OscNoise {
    fn type_name(&self) -> &'static str { "OscNoise" }
    fn ports(&self) -> &'static [PortSpec] { PORTS }
    fn params(&self) -> &'static [ParamSpec] { PARAMS }
    fn display_name(&self) -> &'static str { "Noise Generator" }
    fn category(&self) -> &'static str { "Oscillators" }
}
