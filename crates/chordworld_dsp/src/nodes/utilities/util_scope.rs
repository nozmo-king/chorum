//! Oscilloscope visualization node (passthrough with capture)

use crate::{AudioBusMut, AudioBusRef, DspNode, ParamSpec, PortSpec, ProcessCtx};
use chordworld_core::{EventQueue, ParamIndex, ParamValue};

const BUFFER_SIZE: usize = 1024;

static PORTS: &[PortSpec] = &[PortSpec::audio_in("in", 1), PortSpec::audio_out("out", 1)];
static PARAMS: &[ParamSpec] = &[
    ParamSpec::new("time_div", 1.0, 0.1, 10.0, "ms"), // time division
    ParamSpec::new("trigger_level", 0.0, -1.0, 1.0, ""),
    ParamSpec::new("freeze", 0.0, 0.0, 1.0, ""), // freeze display
];

pub struct UtilScope {
    buffer: [f32; BUFFER_SIZE],
    write_pos: usize,
    time_div: f32,
    trigger_level: f32,
    frozen: bool,
    triggered: bool,
    sample_rate: f32,
}

impl UtilScope {
    pub fn new() -> Self {
        Self {
            buffer: [0.0; BUFFER_SIZE],
            write_pos: 0,
            time_div: 1.0,
            trigger_level: 0.0,
            frozen: false,
            triggered: false,
            sample_rate: 48000.0,
        }
    }

    pub fn get_buffer(&self) -> &[f32] { &self.buffer }
}
impl Default for UtilScope { fn default() -> Self { Self::new() } }

impl DspNode for UtilScope {
    fn reset(&mut self, sr: f32) {
        self.sample_rate = sr;
        self.buffer = [0.0; BUFFER_SIZE];
        self.write_pos = 0;
        self.triggered = false;
    }
    fn process(&mut self, _: &ProcessCtx, ins: &[AudioBusRef], outs: &mut [AudioBusMut]) {
        if ins.is_empty() || outs.is_empty() { return; }
        if let (Some(i), Some(o)) = (ins[0].channel(0), outs[0].channel(0)) {
            let mut prev = if self.write_pos > 0 { self.buffer[self.write_pos - 1] } else { 0.0 };
            for (inp, outp) in i.iter().zip(o.iter_mut()) {
                // Passthrough
                *outp = *inp;

                if !self.frozen {
                    // Simple rising edge trigger
                    if !self.triggered && prev < self.trigger_level && *inp >= self.trigger_level {
                        self.triggered = true;
                        self.write_pos = 0;
                    }

                    if self.triggered || self.trigger_level.abs() < 0.001 {
                        self.buffer[self.write_pos] = *inp;
                        self.write_pos += 1;
                        if self.write_pos >= BUFFER_SIZE {
                            self.write_pos = 0;
                            self.triggered = false;
                        }
                    }
                }
                prev = *inp;
            }
        }
    }
    fn set_param(&mut self, p: ParamIndex, v: ParamValue) {
        match p.0 {
            0 => self.time_div = v.as_float().clamp(0.1, 10.0),
            1 => self.trigger_level = v.as_float().clamp(-1.0, 1.0),
            2 => self.frozen = v.as_float() > 0.5,
            _ => {}
        }
    }
    fn handle_events(&mut self, _: &ProcessCtx, _: &mut EventQueue) {}
}
impl crate::NodeDescriptor for UtilScope {
    fn type_name(&self) -> &'static str { "UtilScope" }
    fn ports(&self) -> &'static [PortSpec] { PORTS }
    fn params(&self) -> &'static [ParamSpec] { PARAMS }
    fn display_name(&self) -> &'static str { "Scope" }
    fn category(&self) -> &'static str { "Utilities" }
}
