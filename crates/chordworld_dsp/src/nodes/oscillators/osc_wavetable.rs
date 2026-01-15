//! Wavetable oscillator with morphing

use crate::{AudioBusMut, AudioBusRef, DspNode, ParamSpec, PortSpec, ProcessCtx};
use chordworld_core::{EventQueue, ParamIndex, ParamValue};

const PARAM_FREQ: u32 = 0;
const PARAM_AMP: u32 = 1;
const PARAM_MORPH: u32 = 2;

const TABLE_SIZE: usize = 2048;

static PORTS: &[PortSpec] = &[PortSpec::audio_out("out", 1)];

static PARAMS: &[ParamSpec] = &[
    ParamSpec::new("freq", 440.0, 20.0, 20000.0, "Hz"),
    ParamSpec::new("amp", 0.5, 0.0, 1.0, ""),
    ParamSpec::new("morph", 0.0, 0.0, 1.0, ""), // Morph between waveforms
];

pub struct OscWavetable {
    phase: f32,
    freq: f32,
    amp: f32,
    morph: f32,
    sample_rate: f32,
    // Built-in wavetables: sine, saw, square, pwm
    tables: [[f32; TABLE_SIZE]; 4],
}

impl OscWavetable {
    pub fn new() -> Self {
        let mut osc = Self {
            phase: 0.0,
            freq: 440.0,
            amp: 0.5,
            morph: 0.0,
            sample_rate: 48000.0,
            tables: [[0.0; TABLE_SIZE]; 4],
        };
        osc.init_tables();
        osc
    }

    fn init_tables(&mut self) {
        use std::f32::consts::TAU;
        for i in 0..TABLE_SIZE {
            let phase = i as f32 / TABLE_SIZE as f32;
            // Sine
            self.tables[0][i] = (phase * TAU).sin();
            // Saw
            self.tables[1][i] = 2.0 * phase - 1.0;
            // Square
            self.tables[2][i] = if phase < 0.5 { 1.0 } else { -1.0 };
            // Triangle
            self.tables[3][i] = 2.0 * (2.0 * phase - 1.0).abs() - 1.0;
        }
    }

    fn read_table(&self, table_idx: usize, phase: f32) -> f32 {
        let pos = phase * TABLE_SIZE as f32;
        let idx0 = pos as usize % TABLE_SIZE;
        let idx1 = (idx0 + 1) % TABLE_SIZE;
        let frac = pos.fract();
        self.tables[table_idx][idx0] * (1.0 - frac) + self.tables[table_idx][idx1] * frac
    }

    fn process_sample(&mut self) -> f32 {
        // Morph between 4 tables: 0-0.33=sine->saw, 0.33-0.66=saw->square, 0.66-1=square->tri
        let morph_scaled = self.morph * 3.0;
        let table_idx = (morph_scaled as usize).min(2);
        let blend = morph_scaled.fract();

        let val0 = self.read_table(table_idx, self.phase);
        let val1 = self.read_table(table_idx + 1, self.phase);
        let output = (val0 * (1.0 - blend) + val1 * blend) * self.amp;

        self.phase += self.freq / self.sample_rate;
        if self.phase >= 1.0 { self.phase -= 1.0; }

        output
    }
}

impl Default for OscWavetable {
    fn default() -> Self { Self::new() }
}

impl DspNode for OscWavetable {
    fn reset(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;
        self.phase = 0.0;
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
            PARAM_MORPH => self.morph = value.as_float().clamp(0.0, 1.0),
            _ => {}
        }
    }

    fn handle_events(&mut self, _ctx: &ProcessCtx, _events: &mut EventQueue) {}
}

impl crate::NodeDescriptor for OscWavetable {
    fn type_name(&self) -> &'static str { "OscWavetable" }
    fn ports(&self) -> &'static [PortSpec] { PORTS }
    fn params(&self) -> &'static [ParamSpec] { PARAMS }
    fn display_name(&self) -> &'static str { "Wavetable Oscillator" }
    fn category(&self) -> &'static str { "Oscillators" }
}
