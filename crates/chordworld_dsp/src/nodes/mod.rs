//! CHORDWORLD DSP Node Library
//!
//! Reorganized for MAXIMALIST expansion:
//! - oscillators/ - Sound generators
//! - utilities/ - Mixers, VCAs, I/O
//!
//! TODO: Add these for full maximalist mode:
//! - filters/ - Signal shaping
//! - envelopes/ - Amplitude/modulation envelopes
//! - effects/ - Audio effects
//! - modulators/ - LFOs, sequencers

pub mod oscillators;
pub mod utilities;

pub use oscillators::*;
pub use utilities::*;

use crate::NodeRegistry;

/// Register all built-in nodes
pub fn register_builtin_nodes(registry: &mut NodeRegistry) {
    // Oscillators
    registry.register("OscSine", || Box::new(oscillators::OscSine::new()));

    // Utilities
    registry.register("Gain", || Box::new(utilities::Gain::new()));
    registry.register("Out", || Box::new(utilities::Out::new()));

    // TODO: Register maximalist node library (see docs/MAXIMALIST_ROADMAP.md)
    // - More oscillators (saw, square, triangle, noise, wavetable, FM)
    // - Filters (SVF, Moog, LP/HP/BP)
    // - Envelopes (ADSR, AR)
    // - Effects (delay, reverb, distortion, chorus)
    // - Modulators (LFO, sequencer)
    // - More utilities (mixer, VCA, split/merge)
}
