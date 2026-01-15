//! Modulation sources

pub mod mod_lfo;
pub mod mod_seq;
pub mod mod_sample_hold;
pub mod mod_env_follower;
pub mod mod_slew;

pub use mod_lfo::ModLFO;
pub use mod_seq::ModSeq;
pub use mod_sample_hold::ModSampleHold;
pub use mod_env_follower::ModEnvFollower;
pub use mod_slew::ModSlew;
