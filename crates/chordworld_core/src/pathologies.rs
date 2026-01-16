//! Temporal & Rhythmic Pathologies
//!
//! Mathematically definable but humanly unperformable musical constructs.
//! These are not jokes; they are hostile edge cases for the composer.

use std::f64::consts::{PI, SQRT_2};

/// A pathology is a transformation that generates "impossible" musical patterns.
#[derive(Debug, Clone)]
pub enum Pathology {
    /// Meter defined as √2/π beats per bar - continuously irrational
    IrrationalMeter { base_bpm: f64 },

    /// Two rhythms with incommensurable periods that never phase-lock
    UnresolvablePolyrhythm { ratio_a: f64, ratio_b: f64 },

    /// Notes accelerate toward infinite density at a finite time
    TemporalEventHorizon { target_time: f64, initial_rate: f64 },

    /// Pitch exists on a continuous manifold, not discrete steps
    ContinuumPitch { drift_rate: f64 },

    /// Every pitch is relative to the previous, no absolute anchor
    DriftingReference { cents_per_step: f64 },

    /// Pitch changes below threshold of conscious detection
    SubPerceptualModulation { modulation_hz: f64, depth_cents: f64 },

    /// Rhythm defined by non-computable sequence approximation
    UncomputableGroove { chaos_seed: u64 },

    /// Algorithm actively sabotages its own patterns
    SelfAdversarial { entropy: f64 },

    /// Listeners hear voices that are not present in the signal
    PhantomPolyphony { fundamental: f64, interval_ratio: f64 },

    /// Music trains expectation then violates at sub-reaction times
    FalsePredictiveTrap { pattern_length: usize, violation_prob: f64 },

    /// Tempo follows function with no closed-form integral
    NonIntegrableTempo { complexity: f64 },

    /// Retroactive alteration - earlier measures changed by later ones
    SelfEditingScore { lookback: usize },
}

impl Pathology {
    /// Get the meter ratio for IrrationalMeter
    pub fn irrational_meter_ratio() -> f64 {
        SQRT_2 / PI // ≈ 0.4502
    }

    /// Calculate the inter-onset interval for temporal event horizon
    /// As t approaches target_time, intervals shrink toward zero
    pub fn event_horizon_ioi(current_time: f64, target_time: f64, initial_rate: f64) -> f64 {
        let remaining = (target_time - current_time).max(0.001);
        initial_rate * remaining.powi(2) / target_time
    }

    /// Generate next pitch in a drifting reference system
    pub fn drifting_pitch(base_freq: f64, cents_offset: f64) -> f64 {
        base_freq * 2.0_f64.powf(cents_offset / 1200.0)
    }

    /// Sub-perceptual modulation - typically 0.1-5Hz, 1-10 cents
    pub fn subperceptual_modulation(base_freq: f64, time: f64, mod_hz: f64, depth_cents: f64) -> f64 {
        let mod_offset = (time * mod_hz * 2.0 * PI).sin() * depth_cents;
        Self::drifting_pitch(base_freq, mod_offset)
    }

    /// Unresolvable polyrhythm - returns phase for both streams
    /// Uses golden ratio approximation for maximal incommensurability
    pub fn polyrhythm_phases(time: f64, base_period: f64) -> (f64, f64) {
        let phi = 1.618033988749895; // Golden ratio
        let phase_a = (time / base_period) % 1.0;
        let phase_b = (time / (base_period * phi)) % 1.0;
        (phase_a, phase_b)
    }

    /// Self-adversarial pattern - deliberately breaks emerging patterns
    pub fn adversarial_next(history: &[u8], entropy: f64, rng_state: &mut u64) -> u8 {
        // Simple PRNG
        *rng_state = rng_state.wrapping_mul(6364136223846793005).wrapping_add(1);
        let random = (*rng_state >> 33) as u8;

        if history.len() < 3 {
            return random;
        }

        // Detect emerging pattern
        let last_three = &history[history.len()-3..];
        let expected_next = if last_three[0] < last_three[1] && last_three[1] < last_three[2] {
            // Ascending - expect continuation
            last_three[2].saturating_add(last_three[2] - last_three[1])
        } else if last_three[0] > last_three[1] && last_three[1] > last_three[2] {
            // Descending - expect continuation
            last_three[2].saturating_sub(last_three[1] - last_three[2])
        } else {
            // No clear pattern
            return random;
        };

        // With probability `entropy`, actively sabotage the expectation
        let threshold = (entropy * 255.0) as u8;
        if random < threshold {
            // Sabotage: return something maximally different
            expected_next.wrapping_add(64 + (random % 64))
        } else {
            expected_next
        }
    }

    /// Phantom polyphony - calculate combination tone frequencies
    /// When two frequencies f1 and f2 are close, you hear f2-f1 and 2f1-f2
    pub fn phantom_tones(f1: f64, f2: f64) -> Vec<f64> {
        let diff = (f2 - f1).abs();
        let cubic_diff = (2.0 * f1 - f2).abs();

        vec![
            diff,                    // Difference tone
            cubic_diff,              // Cubic difference tone
            f1 + f2,                 // Sum tone (usually inaudible)
            3.0 * f1 - 2.0 * f2,     // Higher order
        ].into_iter()
        .filter(|&f| f > 20.0 && f < 20000.0)
        .collect()
    }

    /// Non-integrable tempo function (approximation)
    /// Uses nested sine functions with irrational frequency ratios
    pub fn non_integrable_tempo(time: f64, base_bpm: f64, complexity: f64) -> f64 {
        let phi = 1.618033988749895;
        let e = std::f64::consts::E;

        // Layer multiple incommensurable oscillations
        let mod1 = (time * PI * phi).sin();
        let mod2 = (time * e * 0.5).sin();
        let mod3 = (time * SQRT_2).sin();

        // The integral of this has no closed form
        let modulation = (mod1 * mod2 * mod3) * complexity;

        base_bpm * (1.0 + 0.3 * modulation)
    }

    /// False predictive trap - establish pattern then violate
    pub fn predictive_trap_event(
        step: usize,
        pattern_length: usize,
        violation_prob: f64,
        rng_state: &mut u64,
    ) -> bool {
        // PRNG
        *rng_state = rng_state.wrapping_mul(6364136223846793005).wrapping_add(1);
        let random = (*rng_state >> 32) as f64 / u32::MAX as f64;

        let position_in_pattern = step % pattern_length;

        // At pattern boundary, with violation_prob, skip or double
        if position_in_pattern == 0 && random < violation_prob {
            // Violation: event should NOT occur here
            false
        } else {
            // Normal pattern behavior
            true
        }
    }
}

/// State for running pathology generators
#[derive(Debug, Clone)]
pub struct PathologyState {
    pub pathology: Pathology,
    pub time: f64,
    pub rng_state: u64,
    pub history: Vec<u8>,
    pub pitch_accumulator: f64, // For drifting reference
    pub last_violation: usize,
}

impl PathologyState {
    pub fn new(pathology: Pathology, seed: u64) -> Self {
        Self {
            pathology,
            time: 0.0,
            rng_state: seed,
            history: Vec::new(),
            pitch_accumulator: 0.0,
            last_violation: 0,
        }
    }

    /// Advance time and return next event parameters
    pub fn tick(&mut self, dt: f64) -> PathologyEvent {
        self.time += dt;

        match &self.pathology {
            Pathology::IrrationalMeter { base_bpm } => {
                let beats_per_second = base_bpm / 60.0;
                let irrational_factor = Pathology::irrational_meter_ratio();
                let effective_bps = beats_per_second * irrational_factor;

                PathologyEvent {
                    trigger: (self.time * effective_bps).floor() as u64
                        != ((self.time - dt) * effective_bps).floor() as u64,
                    pitch_hz: None,
                    velocity: Some(0.8),
                    duration: Some(1.0 / effective_bps),
                }
            }

            Pathology::TemporalEventHorizon { target_time, initial_rate } => {
                let ioi = Pathology::event_horizon_ioi(self.time, *target_time, *initial_rate);
                let should_trigger = ioi < dt;

                PathologyEvent {
                    trigger: should_trigger && self.time < *target_time,
                    pitch_hz: None,
                    velocity: Some((1.0 - self.time / target_time) as f32),
                    duration: Some(ioi * 0.5),
                }
            }

            Pathology::ContinuumPitch { drift_rate } => {
                self.pitch_accumulator += drift_rate * dt;
                let base = 440.0;
                let freq = Pathology::drifting_pitch(base, self.pitch_accumulator);

                PathologyEvent {
                    trigger: true, // Continuous
                    pitch_hz: Some(freq),
                    velocity: Some(0.7),
                    duration: None,
                }
            }

            Pathology::SubPerceptualModulation { modulation_hz, depth_cents } => {
                let freq = Pathology::subperceptual_modulation(
                    440.0, self.time, *modulation_hz, *depth_cents
                );

                PathologyEvent {
                    trigger: true,
                    pitch_hz: Some(freq),
                    velocity: Some(0.7),
                    duration: None,
                }
            }

            Pathology::SelfAdversarial { entropy } => {
                let next = Pathology::adversarial_next(&self.history, *entropy, &mut self.rng_state);
                self.history.push(next);
                if self.history.len() > 64 {
                    self.history.remove(0);
                }

                // Map to MIDI note range
                let note = 36 + (next % 48);
                let freq = 440.0 * 2.0_f64.powf((note as f64 - 69.0) / 12.0);

                PathologyEvent {
                    trigger: true,
                    pitch_hz: Some(freq),
                    velocity: Some(next as f32 / 255.0),
                    duration: Some(0.1),
                }
            }

            Pathology::NonIntegrableTempo { complexity } => {
                let tempo = Pathology::non_integrable_tempo(self.time, 120.0, *complexity);
                let period = 60.0 / tempo;

                PathologyEvent {
                    trigger: (self.time % period) < dt,
                    pitch_hz: None,
                    velocity: Some(0.8),
                    duration: Some(period * 0.5),
                }
            }

            _ => PathologyEvent::default(),
        }
    }
}

/// Event output from pathology generator
#[derive(Debug, Clone, Default)]
pub struct PathologyEvent {
    pub trigger: bool,
    pub pitch_hz: Option<f64>,
    pub velocity: Option<f32>,
    pub duration: Option<f64>,
}

/// Pre-defined pathology presets
pub fn presets() -> Vec<(&'static str, Pathology)> {
    vec![
        ("irrational-meter", Pathology::IrrationalMeter { base_bpm: 120.0 }),
        ("golden-polyrhythm", Pathology::UnresolvablePolyrhythm {
            ratio_a: 1.0,
            ratio_b: 1.618033988749895
        }),
        ("event-horizon", Pathology::TemporalEventHorizon {
            target_time: 60.0,
            initial_rate: 1.0
        }),
        ("pitch-drift", Pathology::ContinuumPitch { drift_rate: 5.0 }),
        ("anchorfree", Pathology::DriftingReference { cents_per_step: 7.0 }),
        ("subliminal", Pathology::SubPerceptualModulation {
            modulation_hz: 0.5,
            depth_cents: 5.0
        }),
        ("chaos-groove", Pathology::UncomputableGroove { chaos_seed: 0x21e8 }),
        ("saboteur", Pathology::SelfAdversarial { entropy: 0.7 }),
        ("phantom-voices", Pathology::PhantomPolyphony {
            fundamental: 440.0,
            interval_ratio: 1.5
        }),
        ("trap", Pathology::FalsePredictiveTrap {
            pattern_length: 4,
            violation_prob: 0.3
        }),
        ("unmeasurable", Pathology::NonIntegrableTempo { complexity: 0.8 }),
        ("time-travel", Pathology::SelfEditingScore { lookback: 4 }),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_irrational_meter() {
        let ratio = Pathology::irrational_meter_ratio();
        assert!((ratio - 0.4502).abs() < 0.001);
    }

    #[test]
    fn test_event_horizon_acceleration() {
        let ioi1 = Pathology::event_horizon_ioi(0.0, 10.0, 1.0);
        let ioi2 = Pathology::event_horizon_ioi(5.0, 10.0, 1.0);
        let ioi3 = Pathology::event_horizon_ioi(9.0, 10.0, 1.0);

        // Should accelerate (shorter intervals) as we approach target
        assert!(ioi1 > ioi2);
        assert!(ioi2 > ioi3);
    }

    #[test]
    fn test_phantom_tones() {
        let tones = Pathology::phantom_tones(440.0, 550.0);
        // Should include difference tone (110 Hz)
        assert!(tones.iter().any(|&f| (f - 110.0).abs() < 1.0));
    }

    #[test]
    fn test_adversarial_sabotage() {
        let mut state = 0x21e8u64;
        let ascending = vec![60, 62, 64, 66]; // Clear pattern

        // With high entropy, should usually sabotage
        let mut sabotaged = 0;
        for _ in 0..100 {
            let next = Pathology::adversarial_next(&ascending, 0.9, &mut state);
            // Expected would be 68, sabotaged is something else
            if (next as i16 - 68).abs() > 10 {
                sabotaged += 1;
            }
        }
        // Should sabotage most of the time with 0.9 entropy
        assert!(sabotaged > 70);
    }
}
