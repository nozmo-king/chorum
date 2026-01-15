//! ADSR envelope generator

use crate::{AudioBusMut, AudioBusRef, DspNode, ParamSpec, PortSpec, ProcessCtx};
use chordworld_core::{EventQueue, ParamIndex, ParamValue};

const PARAM_ATTACK: u32 = 0;
const PARAM_DECAY: u32 = 1;
const PARAM_SUSTAIN: u32 = 2;
const PARAM_RELEASE: u32 = 3;
const PARAM_GATE: u32 = 4;

static PORTS: &[PortSpec] = &[PortSpec::audio_out("out", 1)];
static PARAMS: &[ParamSpec] = &[
    ParamSpec::new("attack", 0.01, 0.001, 10.0, "s"),
    ParamSpec::new("decay", 0.1, 0.001, 10.0, "s"),
    ParamSpec::new("sustain", 0.7, 0.0, 1.0, ""),
    ParamSpec::new("release", 0.3, 0.001, 10.0, "s"),
    ParamSpec::new("gate", 0.0, 0.0, 1.0, ""),
];

#[derive(Clone, Copy, PartialEq)]
enum Stage { Idle, Attack, Decay, Sustain, Release }

pub struct EnvADSR {
    attack: f32, decay: f32, sustain: f32, release: f32,
    gate: bool, prev_gate: bool,
    stage: Stage, level: f32, sample_rate: f32,
}

impl EnvADSR {
    pub fn new() -> Self {
        Self {
            attack: 0.01, decay: 0.1, sustain: 0.7, release: 0.3,
            gate: false, prev_gate: false,
            stage: Stage::Idle, level: 0.0, sample_rate: 48000.0,
        }
    }

    fn process_sample(&mut self) -> f32 {
        // Gate edge detection
        if self.gate && !self.prev_gate {
            self.stage = Stage::Attack;
        } else if !self.gate && self.prev_gate {
            self.stage = Stage::Release;
        }
        self.prev_gate = self.gate;

        match self.stage {
            Stage::Idle => {}
            Stage::Attack => {
                let rate = 1.0 / (self.attack * self.sample_rate);
                self.level += rate;
                if self.level >= 1.0 {
                    self.level = 1.0;
                    self.stage = Stage::Decay;
                }
            }
            Stage::Decay => {
                let rate = 1.0 / (self.decay * self.sample_rate);
                self.level -= rate * (1.0 - self.sustain);
                if self.level <= self.sustain {
                    self.level = self.sustain;
                    self.stage = Stage::Sustain;
                }
            }
            Stage::Sustain => {
                self.level = self.sustain;
            }
            Stage::Release => {
                let rate = 1.0 / (self.release * self.sample_rate);
                self.level -= rate * self.level.max(0.001);
                if self.level <= 0.001 {
                    self.level = 0.0;
                    self.stage = Stage::Idle;
                }
            }
        }
        self.level
    }
}

impl Default for EnvADSR { fn default() -> Self { Self::new() } }

impl DspNode for EnvADSR {
    fn reset(&mut self, sr: f32) { self.sample_rate = sr; self.level = 0.0; self.stage = Stage::Idle; }
    fn process(&mut self, _ctx: &ProcessCtx, _ins: &[AudioBusRef], outs: &mut [AudioBusMut]) {
        if outs.is_empty() { return; }
        if let Some(ch) = outs[0].channel(0) {
            for s in ch { *s = self.process_sample(); }
        }
    }
    fn set_param(&mut self, p: ParamIndex, v: ParamValue) {
        match p.0 {
            PARAM_ATTACK => self.attack = v.as_float().max(0.001),
            PARAM_DECAY => self.decay = v.as_float().max(0.001),
            PARAM_SUSTAIN => self.sustain = v.as_float().clamp(0.0, 1.0),
            PARAM_RELEASE => self.release = v.as_float().max(0.001),
            PARAM_GATE => self.gate = v.as_float() > 0.5,
            _ => {}
        }
    }
    fn handle_events(&mut self, _: &ProcessCtx, _: &mut EventQueue) {}
}

impl crate::NodeDescriptor for EnvADSR {
    fn type_name(&self) -> &'static str { "EnvADSR" }
    fn ports(&self) -> &'static [PortSpec] { PORTS }
    fn params(&self) -> &'static [ParamSpec] { PARAMS }
    fn display_name(&self) -> &'static str { "ADSR Envelope" }
    fn category(&self) -> &'static str { "Envelopes" }
}
