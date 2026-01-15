//! CHORDWORLD DSP Node Library
//!
//! Full MAXIMALIST node collection:
//! - oscillators/ - Sound generators (sine, saw, square, triangle, noise, wavetable, FM)
//! - filters/ - Signal shaping (SVF, Moog, LP, HP, BP, Notch)
//! - envelopes/ - Amplitude/modulation envelopes (ADSR, AR, Multi)
//! - effects/ - Audio effects (delay, reverb, distortion, chorus, phaser, flanger, compressor, EQ)
//! - modulators/ - LFOs, sequencers, S&H, envelope follower, slew
//! - utilities/ - Mixers, VCAs, I/O, split/merge, quantizer, scope

pub mod oscillators;
pub mod filters;
pub mod envelopes;
pub mod effects;
pub mod modulators;
pub mod utilities;

pub use oscillators::*;
pub use filters::*;
pub use envelopes::*;
pub use effects::*;
pub use modulators::*;
pub use utilities::*;

use crate::NodeRegistry;

/// Register all built-in nodes
pub fn register_builtin_nodes(registry: &mut NodeRegistry) {
    // Oscillators
    registry.register("OscSine", || Box::new(oscillators::OscSine::new()));
    registry.register("OscSaw", || Box::new(oscillators::OscSaw::new()));
    registry.register("OscSquare", || Box::new(oscillators::OscSquare::new()));
    registry.register("OscTriangle", || Box::new(oscillators::OscTriangle::new()));
    registry.register("OscNoise", || Box::new(oscillators::OscNoise::new()));
    registry.register("OscWavetable", || Box::new(oscillators::OscWavetable::new()));
    registry.register("OscFM", || Box::new(oscillators::OscFM::new()));

    // Filters
    registry.register("FilterSVF", || Box::new(filters::FilterSVF::new()));
    registry.register("FilterMoog", || Box::new(filters::FilterMoog::new()));
    registry.register("FilterLP", || Box::new(filters::FilterLP::new()));
    registry.register("FilterHP", || Box::new(filters::FilterHP::new()));
    registry.register("FilterBP", || Box::new(filters::FilterBP::new()));
    registry.register("FilterNotch", || Box::new(filters::FilterNotch::new()));

    // Envelopes
    registry.register("EnvADSR", || Box::new(envelopes::EnvADSR::new()));
    registry.register("EnvAR", || Box::new(envelopes::EnvAR::new()));
    registry.register("EnvMulti", || Box::new(envelopes::EnvMulti::new()));

    // Effects
    registry.register("FxDelay", || Box::new(effects::FxDelay::new()));
    registry.register("FxReverb", || Box::new(effects::FxReverb::new()));
    registry.register("FxDistortion", || Box::new(effects::FxDistortion::new()));
    registry.register("FxChorus", || Box::new(effects::FxChorus::new()));
    registry.register("FxPhaser", || Box::new(effects::FxPhaser::new()));
    registry.register("FxFlanger", || Box::new(effects::FxFlanger::new()));
    registry.register("FxCompressor", || Box::new(effects::FxCompressor::new()));
    registry.register("FxEQ", || Box::new(effects::FxEQ::new()));

    // Modulators
    registry.register("ModLFO", || Box::new(modulators::ModLFO::new()));
    registry.register("ModSeq", || Box::new(modulators::ModSeq::new()));
    registry.register("ModSampleHold", || Box::new(modulators::ModSampleHold::new()));
    registry.register("ModEnvFollower", || Box::new(modulators::ModEnvFollower::new()));
    registry.register("ModSlew", || Box::new(modulators::ModSlew::new()));

    // Utilities
    registry.register("Gain", || Box::new(utilities::Gain::new()));
    registry.register("Out", || Box::new(utilities::Out::new()));
    registry.register("UtilMixer", || Box::new(utilities::UtilMixer::new()));
    registry.register("UtilVCA", || Box::new(utilities::UtilVCA::new()));
    registry.register("UtilSplit", || Box::new(utilities::UtilSplit::new()));
    registry.register("UtilMerge", || Box::new(utilities::UtilMerge::new()));
    registry.register("UtilQuantizer", || Box::new(utilities::UtilQuantizer::new()));
    registry.register("UtilScope", || Box::new(utilities::UtilScope::new()));
}
