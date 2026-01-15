//! Multi-stage envelope (up to 8 stages)

use crate::{AudioBusMut, AudioBusRef, DspNode, ParamSpec, PortSpec, ProcessCtx};
use chordworld_core::{EventQueue, ParamIndex, ParamValue};

const MAX_STAGES: usize = 8;

static PORTS: &[PortSpec] = &[PortSpec::audio_out("out", 1)];
static PARAMS: &[ParamSpec] = &[
    ParamSpec::new("stages", 4.0, 1.0, 8.0, ""),
    ParamSpec::new("time1", 0.1, 0.001, 10.0, "s"),
    ParamSpec::new("level1", 1.0, 0.0, 1.0, ""),
    ParamSpec::new("time2", 0.2, 0.001, 10.0, "s"),
    ParamSpec::new("level2", 0.7, 0.0, 1.0, ""),
    ParamSpec::new("time3", 0.3, 0.001, 10.0, "s"),
    ParamSpec::new("level3", 0.5, 0.0, 1.0, ""),
    ParamSpec::new("time4", 0.5, 0.001, 10.0, "s"),
    ParamSpec::new("level4", 0.0, 0.0, 1.0, ""),
    ParamSpec::new("gate", 0.0, 0.0, 1.0, ""),
];

pub struct EnvMulti {
    num_stages: usize,
    times: [f32; MAX_STAGES],
    levels: [f32; MAX_STAGES],
    gate: bool, prev_gate: bool,
    current_stage: usize,
    level: f32, stage_progress: f32,
    sample_rate: f32,
}

impl EnvMulti {
    pub fn new() -> Self {
        Self {
            num_stages: 4,
            times: [0.1, 0.2, 0.3, 0.5, 0.1, 0.1, 0.1, 0.1],
            levels: [1.0, 0.7, 0.5, 0.0, 0.0, 0.0, 0.0, 0.0],
            gate: false, prev_gate: false,
            current_stage: 0, level: 0.0, stage_progress: 0.0,
            sample_rate: 48000.0,
        }
    }

    fn process_sample(&mut self) -> f32 {
        if self.gate && !self.prev_gate {
            self.current_stage = 0;
            self.stage_progress = 0.0;
        }
        self.prev_gate = self.gate;

        if self.current_stage < self.num_stages && self.gate {
            let target = self.levels[self.current_stage];
            let time = self.times[self.current_stage];
            let rate = 1.0 / (time * self.sample_rate);
            self.stage_progress += rate;

            let prev_level = if self.current_stage == 0 { 0.0 } else { self.levels[self.current_stage - 1] };
            self.level = prev_level + (target - prev_level) * self.stage_progress.min(1.0);

            if self.stage_progress >= 1.0 {
                self.current_stage += 1;
                self.stage_progress = 0.0;
            }
        } else if !self.gate && self.level > 0.0 {
            self.level *= 0.9995; // Quick release
            if self.level < 0.001 { self.level = 0.0; }
        }
        self.level
    }
}

impl Default for EnvMulti { fn default() -> Self { Self::new() } }

impl DspNode for EnvMulti {
    fn reset(&mut self, sr: f32) { self.sample_rate = sr; self.level = 0.0; self.current_stage = 0; }
    fn process(&mut self, _: &ProcessCtx, _: &[AudioBusRef], outs: &mut [AudioBusMut]) {
        if let Some(ch) = outs.get_mut(0).and_then(|o| o.channel(0)) { for s in ch { *s = self.process_sample(); } }
    }
    fn set_param(&mut self, p: ParamIndex, v: ParamValue) {
        match p.0 {
            0 => self.num_stages = (v.as_float() as usize).clamp(1, MAX_STAGES),
            1 => self.times[0] = v.as_float().max(0.001),
            2 => self.levels[0] = v.as_float().clamp(0.0, 1.0),
            3 => self.times[1] = v.as_float().max(0.001),
            4 => self.levels[1] = v.as_float().clamp(0.0, 1.0),
            5 => self.times[2] = v.as_float().max(0.001),
            6 => self.levels[2] = v.as_float().clamp(0.0, 1.0),
            7 => self.times[3] = v.as_float().max(0.001),
            8 => self.levels[3] = v.as_float().clamp(0.0, 1.0),
            9 => self.gate = v.as_float() > 0.5,
            _ => {}
        }
    }
    fn handle_events(&mut self, _: &ProcessCtx, _: &mut EventQueue) {}
}

impl crate::NodeDescriptor for EnvMulti {
    fn type_name(&self) -> &'static str { "EnvMulti" }
    fn ports(&self) -> &'static [PortSpec] { PORTS }
    fn params(&self) -> &'static [ParamSpec] { PARAMS }
    fn display_name(&self) -> &'static str { "Multi-Stage Envelope" }
    fn category(&self) -> &'static str { "Envelopes" }
}
