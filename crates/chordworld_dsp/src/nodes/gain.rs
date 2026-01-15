//! Gain/amplitude control node

use crate::{AudioBusMut, AudioBusRef, DspNode, ParamSpec, PortSpec, ProcessCtx};
use chordworld_core::{EventQueue, ParamIndex, ParamValue};

const PARAM_GAIN: u32 = 0;

static PORTS: &[PortSpec] = &[
    PortSpec::audio_in("in", 2),
    PortSpec::audio_out("out", 2),
];

static PARAMS: &[ParamSpec] = &[ParamSpec::new("gain", 1.0, 0.0, 2.0, "")];

pub struct Gain {
    gain: f32,
}

impl Gain {
    pub fn new() -> Self {
        Self { gain: 1.0 }
    }
}

impl Default for Gain {
    fn default() -> Self {
        Self::new()
    }
}

impl DspNode for Gain {
    fn reset(&mut self, _sample_rate: f32) {
        // No state to reset
    }

    fn process(&mut self, _ctx: &ProcessCtx, ins: &[AudioBusRef], outs: &mut [AudioBusMut]) {
        if ins.is_empty() || outs.is_empty() {
            return;
        }

        let input = &ins[0];
        let output = &mut outs[0];

        // Process all channels
        let channel_count = input.channel_count().min(output.channel_count());
        for ch in 0..channel_count {
            if let (Some(in_channel), Some(out_channel)) = (input.channel(ch), output.channel(ch)) {
                for (in_sample, out_sample) in in_channel.iter().zip(out_channel.iter_mut()) {
                    *out_sample = *in_sample * self.gain;
                }
            }
        }
    }

    fn set_param(&mut self, param: ParamIndex, value: ParamValue) {
        match param.0 {
            PARAM_GAIN => self.gain = value.as_float(),
            _ => {}
        }
    }

    fn handle_events(&mut self, _ctx: &ProcessCtx, _events: &mut EventQueue) {
        // No event handling
    }
}

impl crate::NodeDescriptor for Gain {
    fn type_name(&self) -> &'static str {
        "Gain"
    }

    fn ports(&self) -> &'static [PortSpec] {
        PORTS
    }

    fn params(&self) -> &'static [ParamSpec] {
        PARAMS
    }

    fn display_name(&self) -> &'static str {
        "Gain"
    }

    fn category(&self) -> &'static str {
        "Utilities"
    }
}
