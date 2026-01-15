//! Built-in DSP nodes

pub mod osc_sine;
pub mod gain;
pub mod out;

pub use osc_sine::OscSine;
pub use gain::Gain;
pub use out::Out;

use crate::NodeRegistry;

/// Register all built-in nodes
pub fn register_builtin_nodes(registry: &mut NodeRegistry) {
    registry.register("OscSine", || Box::new(OscSine::new()));
    registry.register("Gain", || Box::new(Gain::new()));
    registry.register("Out", || Box::new(Out::new()));
}
