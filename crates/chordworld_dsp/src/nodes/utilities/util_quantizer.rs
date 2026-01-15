//! Pitch quantizer (snaps CV to scale)

use crate::{AudioBusMut, AudioBusRef, DspNode, ParamSpec, PortSpec, ProcessCtx};
use chordworld_core::{EventQueue, ParamIndex, ParamValue};

static PORTS: &[PortSpec] = &[PortSpec::audio_in("in", 1), PortSpec::audio_out("out", 1)];
static PARAMS: &[ParamSpec] = &[
    ParamSpec::new("scale", 0.0, 0.0, 7.0, ""), // 0=chromatic, 1=major, 2=minor, 3=pentatonic, 4=blues, 5=dorian, 6=phrygian, 7=mixolydian
    ParamSpec::new("root", 0.0, 0.0, 11.0, ""), // root note (0=C, 1=C#, etc.)
    ParamSpec::new("strength", 1.0, 0.0, 1.0, ""), // quantization strength (0=bypass, 1=full)
];

// Scale intervals in semitones from root
const CHROMATIC: &[f32] = &[0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0];
const MAJOR: &[f32] = &[0.0, 2.0, 4.0, 5.0, 7.0, 9.0, 11.0];
const MINOR: &[f32] = &[0.0, 2.0, 3.0, 5.0, 7.0, 8.0, 10.0];
const PENTATONIC: &[f32] = &[0.0, 2.0, 4.0, 7.0, 9.0];
const BLUES: &[f32] = &[0.0, 3.0, 5.0, 6.0, 7.0, 10.0];
const DORIAN: &[f32] = &[0.0, 2.0, 3.0, 5.0, 7.0, 9.0, 10.0];
const PHRYGIAN: &[f32] = &[0.0, 1.0, 3.0, 5.0, 7.0, 8.0, 10.0];
const MIXOLYDIAN: &[f32] = &[0.0, 2.0, 4.0, 5.0, 7.0, 9.0, 10.0];

pub struct UtilQuantizer {
    scale: u32,
    root: f32,
    strength: f32,
}

impl UtilQuantizer {
    pub fn new() -> Self { Self { scale: 0, root: 0.0, strength: 1.0 } }

    fn get_scale(&self) -> &'static [f32] {
        match self.scale {
            0 => CHROMATIC, 1 => MAJOR, 2 => MINOR, 3 => PENTATONIC,
            4 => BLUES, 5 => DORIAN, 6 => PHRYGIAN, 7 => MIXOLYDIAN,
            _ => CHROMATIC,
        }
    }

    fn quantize(&self, input: f32) -> f32 {
        // Input is expected to be in V/Oct (1V = 1 octave)
        let scale = self.get_scale();
        let semitones = input * 12.0;
        let shifted = semitones - self.root;
        let octave = (shifted / 12.0).floor();
        let note_in_octave = shifted - octave * 12.0;

        // Find nearest scale degree
        let mut nearest = scale[0];
        let mut min_dist = (note_in_octave - scale[0]).abs();
        for &degree in scale.iter().skip(1) {
            let dist = (note_in_octave - degree).abs();
            if dist < min_dist { min_dist = dist; nearest = degree; }
        }
        // Also check wrap-around to next octave
        let wrap_dist = (note_in_octave - (scale[0] + 12.0)).abs();
        if wrap_dist < min_dist { nearest = scale[0] + 12.0; }

        let quantized = (octave * 12.0 + nearest + self.root) / 12.0;
        input + (quantized - input) * self.strength
    }
}
impl Default for UtilQuantizer { fn default() -> Self { Self::new() } }

impl DspNode for UtilQuantizer {
    fn reset(&mut self, _: f32) {}
    fn process(&mut self, _: &ProcessCtx, ins: &[AudioBusRef], outs: &mut [AudioBusMut]) {
        if ins.is_empty() || outs.is_empty() { return; }
        if let (Some(i), Some(o)) = (ins[0].channel(0), outs[0].channel(0)) {
            for (inp, outp) in i.iter().zip(o.iter_mut()) {
                *outp = self.quantize(*inp);
            }
        }
    }
    fn set_param(&mut self, p: ParamIndex, v: ParamValue) {
        match p.0 {
            0 => self.scale = (v.as_float() as u32).min(7),
            1 => self.root = v.as_float().clamp(0.0, 11.0),
            2 => self.strength = v.as_float().clamp(0.0, 1.0),
            _ => {}
        }
    }
    fn handle_events(&mut self, _: &ProcessCtx, _: &mut EventQueue) {}
}
impl crate::NodeDescriptor for UtilQuantizer {
    fn type_name(&self) -> &'static str { "UtilQuantizer" }
    fn ports(&self) -> &'static [PortSpec] { PORTS }
    fn params(&self) -> &'static [ParamSpec] { PARAMS }
    fn display_name(&self) -> &'static str { "Quantizer" }
    fn category(&self) -> &'static str { "Utilities" }
}
