//! AR envelope (percussion/trigger style)

use crate::{AudioBusMut, AudioBusRef, DspNode, ParamSpec, PortSpec, ProcessCtx};
use chordworld_core::{EventQueue, ParamIndex, ParamValue};

static PORTS: &[PortSpec] = &[PortSpec::audio_out("out", 1)];
static PARAMS: &[ParamSpec] = &[
    ParamSpec::new("attack", 0.001, 0.0001, 5.0, "s"),
    ParamSpec::new("release", 0.1, 0.001, 10.0, "s"),
    ParamSpec::new("trigger", 0.0, 0.0, 1.0, ""),
];

pub struct EnvAR {
    attack: f32, release: f32,
    triggered: bool, prev_trig: bool,
    in_attack: bool, level: f32, sample_rate: f32,
}

impl EnvAR {
    pub fn new() -> Self {
        Self { attack: 0.001, release: 0.1, triggered: false, prev_trig: false, in_attack: false, level: 0.0, sample_rate: 48000.0 }
    }
    fn process_sample(&mut self) -> f32 {
        if self.triggered && !self.prev_trig { self.in_attack = true; }
        self.prev_trig = self.triggered;
        if self.in_attack {
            self.level += 1.0 / (self.attack * self.sample_rate);
            if self.level >= 1.0 { self.level = 1.0; self.in_attack = false; }
        } else if self.level > 0.0 {
            self.level -= 1.0 / (self.release * self.sample_rate);
            if self.level < 0.0 { self.level = 0.0; }
        }
        self.level
    }
}
impl Default for EnvAR { fn default() -> Self { Self::new() } }

impl DspNode for EnvAR {
    fn reset(&mut self, sr: f32) { self.sample_rate = sr; self.level = 0.0; self.in_attack = false; }
    fn process(&mut self, _: &ProcessCtx, _: &[AudioBusRef], outs: &mut [AudioBusMut]) {
        if let Some(ch) = outs.get_mut(0).and_then(|o| o.channel(0)) { for s in ch { *s = self.process_sample(); } }
    }
    fn set_param(&mut self, p: ParamIndex, v: ParamValue) {
        match p.0 { 0 => self.attack = v.as_float().max(0.0001), 1 => self.release = v.as_float().max(0.001), 2 => self.triggered = v.as_float() > 0.5, _ => {} }
    }
    fn handle_events(&mut self, _: &ProcessCtx, _: &mut EventQueue) {}
}
impl crate::NodeDescriptor for EnvAR {
    fn type_name(&self) -> &'static str { "EnvAR" }
    fn ports(&self) -> &'static [PortSpec] { PORTS }
    fn params(&self) -> &'static [ParamSpec] { PARAMS }
    fn display_name(&self) -> &'static str { "AR Envelope" }
    fn category(&self) -> &'static str { "Envelopes" }
}
