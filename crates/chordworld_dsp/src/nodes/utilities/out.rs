//! Output node (audio interface output)

use crate::{AudioBusMut, AudioBusRef, DspNode, ParamSpec, PortSpec, ProcessCtx};
use chordworld_core::{EventQueue, ParamIndex, ParamValue};

static PORTS: &[PortSpec] = &[
    PortSpec::audio_in("in", 2),
    PortSpec::audio_out("out", 2),
];

static PARAMS: &[ParamSpec] = &[];

pub struct Out {
    // Output nodes typically don't process, they just mark the endpoint
    // The engine will route this to the device output
}

impl Out {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for Out {
    fn default() -> Self {
        Self::new()
    }
}

impl DspNode for Out {
    fn reset(&mut self, _sample_rate: f32) {
        // No state
    }

    fn process(&mut self, _ctx: &ProcessCtx, ins: &[AudioBusRef], outs: &mut [AudioBusMut]) {
        // Copy input to output (passthrough)
        if ins.is_empty() || outs.is_empty() {
            return;
        }

        let input = &ins[0];
        let output = &mut outs[0];

        let channel_count = input.channel_count().min(output.channel_count());
        for ch in 0..channel_count {
            if let (Some(in_channel), Some(out_channel)) = (input.channel(ch), output.channel(ch)) {
                out_channel.copy_from_slice(in_channel);
            }
        }
    }

    fn set_param(&mut self, _param: ParamIndex, _value: ParamValue) {
        // No parameters
    }

    fn handle_events(&mut self, _ctx: &ProcessCtx, _events: &mut EventQueue) {
        // No event handling
    }
}

impl crate::NodeDescriptor for Out {
    fn type_name(&self) -> &'static str {
        "Out"
    }

    fn ports(&self) -> &'static [PortSpec] {
        PORTS
    }

    fn params(&self) -> &'static [ParamSpec] {
        PARAMS
    }

    fn display_name(&self) -> &'static str {
        "Audio Output"
    }

    fn category(&self) -> &'static str {
        "I/O"
    }
}
