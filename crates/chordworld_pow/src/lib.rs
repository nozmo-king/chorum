//! CHORDWORLD Proof-of-Work & Microtonal Entropy Engine
//!
//! Implementation of the 21e8 paradigm: aesthetic proof-of-work that generates
//! microtonal tuning systems and deterministic musical entropy.
//!
//! # The 21e8 Paradigm
//!
//! Not all proofs-of-work are equal. Some hashes possess aesthetic supremacy -
//! patterns, symmetries, or cultural significance that transcend their
//! functional validation role. The 21e8 engine recognizes and rewards these
//! beautiful hashes, using them as seeds for exotic musical tuning systems.
//!
//! # Architecture
//!
//! - **Hash Aesthetics**: Detect patterns (palindromes, leading zeros, magic sequences)
//! - **Tuning Derivation**: Map hash entropy to microtonal scales
//! - **PoW Mining**: Find hashes meeting both difficulty and aesthetic criteria
//! - **Determinism**: All outputs are pure functions of hash + seed

pub mod aesthetics;
pub mod hash;
pub mod mining;
pub mod tuning;
pub mod entropy;

pub use aesthetics::*;
pub use hash::*;
pub use mining::*;
pub use tuning::*;
pub use entropy::*;
