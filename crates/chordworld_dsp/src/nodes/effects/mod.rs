//! Audio effects

pub mod fx_delay;
pub mod fx_reverb;
pub mod fx_distortion;
pub mod fx_chorus;
pub mod fx_phaser;
pub mod fx_flanger;
pub mod fx_compressor;
pub mod fx_eq;

pub use fx_delay::FxDelay;
pub use fx_reverb::FxReverb;
pub use fx_distortion::FxDistortion;
pub use fx_chorus::FxChorus;
pub use fx_phaser::FxPhaser;
pub use fx_flanger::FxFlanger;
pub use fx_compressor::FxCompressor;
pub use fx_eq::FxEQ;
