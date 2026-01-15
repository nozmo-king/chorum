//! Envelope generators

pub mod env_adsr;
pub mod env_ar;
pub mod env_multi;

pub use env_adsr::EnvADSR;
pub use env_ar::EnvAR;
pub use env_multi::EnvMulti;
