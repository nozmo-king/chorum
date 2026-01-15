//! Hash-derived microtonal tuning systems
//!
//! The aesthetic supremacy of a hash determines the exotic tuning system it generates.
//! Beautiful hashes create beautiful scales.

use crate::Hash256;
use serde::{Deserialize, Serialize};

/// A microtonal scale derived from hash entropy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MicrotonalScale {
    /// Name/identifier for this scale
    pub name: String,

    /// Pitches in cents relative to root (12-TET middle C = 0)
    pub pitches_cents: Vec<f32>,

    /// Number of divisions per octave (EDO = Equal Division of Octave)
    pub divisions: u32,

    /// The hash this scale was derived from
    pub source_hash: Hash256,

    /// Aesthetic score of the source hash
    pub aesthetic_score: u32,
}

impl MicrotonalScale {
    /// Derive a microtonal scale from a hash
    pub fn from_hash(hash: &Hash256) -> Self {
        let bytes = hash.as_bytes();

        // Use hash entropy to determine scale properties
        let divisions = Self::hash_to_divisions(bytes);
        let octave_cents = 1200.0; // Standard octave

        // Generate pitches using hash entropy
        let mut pitches_cents = Vec::new();
        for i in 0..divisions {
            let cents = (i as f32 / divisions as f32) * octave_cents;

            // Add micro-detuning based on hash entropy
            let detune_byte = bytes[(i as usize) % 32];
            let detune = ((detune_byte as f32 / 255.0) - 0.5) * 5.0; // ±2.5 cents

            pitches_cents.push(cents + detune);
        }

        // Generate name from hash prefix
        let name = format!("21e8-{}", &hash.to_hex()[..8]);

        Self {
            name,
            pitches_cents,
            divisions,
            source_hash: *hash,
            aesthetic_score: 0, // Will be filled by caller
        }
    }

    /// Map hash bytes to number of divisions (5-72 EDO range)
    fn hash_to_divisions(bytes: &[u8; 32]) -> u32 {
        // Use first byte to determine divisions
        let byte = bytes[0];

        // Map to musically useful divisions
        // Common microtonal systems: 19, 22, 24, 31, 41, 43, 53, 72 EDO
        let common_systems = [19, 22, 24, 31, 41, 43, 53, 72];
        let index = (byte as usize) % common_systems.len();
        common_systems[index]
    }

    /// Get frequency ratio for a scale degree
    pub fn get_ratio(&self, degree: u32) -> f32 {
        let index = (degree as usize) % self.pitches_cents.len();
        let cents = self.pitches_cents[index];

        // Convert cents to ratio: ratio = 2^(cents/1200)
        2.0f32.powf(cents / 1200.0)
    }

    /// Get frequency in Hz for a scale degree relative to a root frequency
    pub fn get_frequency(&self, degree: u32, root_hz: f32) -> f32 {
        root_hz * self.get_ratio(degree)
    }

    /// Quantize a frequency to the nearest scale degree
    pub fn quantize_frequency(&self, freq_hz: f32, root_hz: f32) -> f32 {
        let ratio = freq_hz / root_hz;
        let cents = 1200.0 * ratio.log2();

        // Find nearest scale degree
        let nearest_degree = self
            .pitches_cents
            .iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| {
                let dist_a = (cents - **a).abs();
                let dist_b = (cents - **b).abs();
                dist_a.partial_cmp(&dist_b).unwrap()
            })
            .map(|(i, _)| i)
            .unwrap_or(0);

        self.get_frequency(nearest_degree as u32, root_hz)
    }
}

/// Tuning system type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TuningSystem {
    /// Standard 12-tone equal temperament
    TwelveTET,

    /// Just intonation ratios
    JustIntonation,

    /// Equal division of octave (EDO)
    EDO(u32),

    /// Hash-derived microtonal system
    HashDerived,

    /// Bohlen-Pierce scale (tritave-based, 13 divisions)
    BohlenPierce,

    /// Wendy Carlos Alpha scale
    CarlosAlpha,

    /// Pythagorean tuning
    Pythagorean,
}

/// Factory for creating tuning systems
pub struct TuningFactory;

impl TuningFactory {
    /// Create a 12-TET scale (standard Western tuning)
    pub fn twelve_tet() -> MicrotonalScale {
        let mut pitches_cents = Vec::new();
        for i in 0..12 {
            pitches_cents.push(i as f32 * 100.0);
        }

        MicrotonalScale {
            name: "12-TET".to_string(),
            pitches_cents,
            divisions: 12,
            source_hash: Hash256::zero(),
            aesthetic_score: 0,
        }
    }

    /// Create a 19-EDO scale (meantone approximation)
    pub fn nineteen_edo() -> MicrotonalScale {
        let mut pitches_cents = Vec::new();
        let step = 1200.0 / 19.0;
        for i in 0..19 {
            pitches_cents.push(i as f32 * step);
        }

        MicrotonalScale {
            name: "19-EDO".to_string(),
            pitches_cents,
            divisions: 19,
            source_hash: Hash256::zero(),
            aesthetic_score: 0,
        }
    }

    /// Create a 31-EDO scale (excellent fifth approximation)
    pub fn thirtyone_edo() -> MicrotonalScale {
        let mut pitches_cents = Vec::new();
        let step = 1200.0 / 31.0;
        for i in 0..31 {
            pitches_cents.push(i as f32 * step);
        }

        MicrotonalScale {
            name: "31-EDO".to_string(),
            pitches_cents,
            divisions: 31,
            source_hash: Hash256::zero(),
            aesthetic_score: 0,
        }
    }

    /// Create a 53-EDO scale (Pythagorean comma division)
    pub fn fiftythree_edo() -> MicrotonalScale {
        let mut pitches_cents = Vec::new();
        let step = 1200.0 / 53.0;
        for i in 0..53 {
            pitches_cents.push(i as f32 * step);
        }

        MicrotonalScale {
            name: "53-EDO".to_string(),
            pitches_cents,
            divisions: 53,
            source_hash: Hash256::zero(),
            aesthetic_score: 0,
        }
    }

    /// Create a Bohlen-Pierce scale (13 divisions of a tritave = 3:1)
    pub fn bohlen_pierce() -> MicrotonalScale {
        let mut pitches_cents = Vec::new();
        let tritave_cents = 1200.0 * (3.0f32).log2(); // ~1901.96 cents
        let step = tritave_cents / 13.0;

        for i in 0..13 {
            pitches_cents.push(i as f32 * step);
        }

        MicrotonalScale {
            name: "Bohlen-Pierce".to_string(),
            pitches_cents,
            divisions: 13,
            source_hash: Hash256::zero(),
            aesthetic_score: 0,
        }
    }

    /// Create a Just Intonation major scale
    pub fn just_intonation_major() -> MicrotonalScale {
        // Ratios: 1/1, 9/8, 5/4, 4/3, 3/2, 5/3, 15/8, 2/1
        let ratios: [f32; 8] = [1.0, 9.0 / 8.0, 5.0 / 4.0, 4.0 / 3.0, 3.0 / 2.0, 5.0 / 3.0, 15.0 / 8.0, 2.0];

        let pitches_cents: Vec<f32> = ratios.iter().map(|r| 1200.0 * r.log2()).collect();

        MicrotonalScale {
            name: "Just Intonation Major".to_string(),
            pitches_cents,
            divisions: 7,
            source_hash: Hash256::zero(),
            aesthetic_score: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_twelve_tet() {
        let scale = TuningFactory::twelve_tet();
        assert_eq!(scale.divisions, 12);

        // A440 should be close to expected
        let a440 = scale.get_frequency(9, 261.63); // C4 = 261.63 Hz
        assert!((a440 - 440.0).abs() < 1.0);
    }

    #[test]
    fn test_hash_derived_scale() {
        let hash = Hash256::zero();
        let scale = MicrotonalScale::from_hash(&hash);

        assert!(!scale.pitches_cents.is_empty());
        assert!(scale.divisions >= 5 && scale.divisions <= 72);
    }

    #[test]
    fn test_ratio_to_cents() {
        let scale = TuningFactory::twelve_tet();

        // Octave (2:1 ratio) should be 1200 cents
        let octave_ratio = scale.get_ratio(12);
        assert!((octave_ratio - 2.0).abs() < 0.01);
    }
}
