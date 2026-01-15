//! Oscillator nodes

pub mod osc_sine;
pub mod osc_saw;
pub mod osc_square;
pub mod osc_triangle;
pub mod osc_noise;
pub mod osc_wavetable;
pub mod osc_fm;

pub use osc_sine::OscSine;
pub use osc_saw::OscSaw;
pub use osc_square::OscSquare;
pub use osc_triangle::OscTriangle;
pub use osc_noise::OscNoise;
pub use osc_wavetable::OscWavetable;
pub use osc_fm::OscFM;
