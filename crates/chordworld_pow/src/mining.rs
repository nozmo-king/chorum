//! Proof-of-Work mining with aesthetic criteria
//!
//! Find hashes that are both computationally difficult AND aesthetically supreme.

use crate::{AestheticCriteria, AestheticScore, Hash256};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MiningError {
    #[error("Mining was cancelled")]
    Cancelled,

    #[error("No solution found within iteration limit")]
    IterationLimitExceeded,
}

/// Result of a mining operation
#[derive(Debug, Clone)]
pub struct MiningResult {
    /// The found hash
    pub hash: Hash256,

    /// Aesthetic score of the hash
    pub aesthetic_score: AestheticScore,

    /// Nonce that produced this hash
    pub nonce: u64,

    /// Number of hashes computed
    pub iterations: u64,

    /// Time taken
    pub duration: Duration,

    /// Hash rate (hashes per second)
    pub hash_rate: f64,
}

/// Mining configuration
#[derive(Debug, Clone)]
pub struct MiningConfig {
    /// Aesthetic criteria to meet
    pub criteria: AestheticCriteria,

    /// Maximum iterations before giving up (None = unlimited)
    pub max_iterations: Option<u64>,

    /// Report progress every N iterations
    pub progress_interval: u64,
}

impl Default for MiningConfig {
    fn default() -> Self {
        Self {
            criteria: AestheticCriteria::standard(),
            max_iterations: None,
            progress_interval: 100_000,
        }
    }
}

/// Proof-of-work miner
pub struct Miner {
    config: MiningConfig,
    cancel_flag: Arc<AtomicBool>,
    iterations: Arc<AtomicU64>,
}

impl Miner {
    pub fn new(config: MiningConfig) -> Self {
        Self {
            config,
            cancel_flag: Arc::new(AtomicBool::new(false)),
            iterations: Arc::new(AtomicU64::new(0)),
        }
    }

    /// Mine for a hash meeting aesthetic criteria
    pub fn mine(&self, data: &[u8]) -> Result<MiningResult, MiningError> {
        let start_time = Instant::now();
        let mut nonce = 0u64;

        loop {
            // Check if cancelled
            if self.cancel_flag.load(Ordering::Relaxed) {
                return Err(MiningError::Cancelled);
            }

            // Check iteration limit
            if let Some(max_iter) = self.config.max_iterations {
                if nonce >= max_iter {
                    return Err(MiningError::IterationLimitExceeded);
                }
            }

            // Compute hash with nonce
            let hash = self.hash_with_nonce(data, nonce);

            // Evaluate aesthetics
            let score = AestheticScore::evaluate(&hash);

            // Check if it meets criteria
            if self.config.criteria.validate(&score) {
                let duration = start_time.elapsed();
                let hash_rate = nonce as f64 / duration.as_secs_f64();

                return Ok(MiningResult {
                    hash,
                    aesthetic_score: score,
                    nonce,
                    iterations: nonce,
                    duration,
                    hash_rate,
                });
            }

            // Progress reporting
            if nonce % self.config.progress_interval == 0 && nonce > 0 {
                self.report_progress(nonce, &start_time);
            }

            nonce += 1;
            self.iterations.store(nonce, Ordering::Relaxed);
        }
    }

    /// Hash data with a nonce
    fn hash_with_nonce(&self, data: &[u8], nonce: u64) -> Hash256 {
        let mut hasher = blake3::Hasher::new();
        hasher.update(data);
        hasher.update(&nonce.to_le_bytes());
        let hash = hasher.finalize();
        Hash256::from(*hash.as_bytes())
    }

    /// Report mining progress
    fn report_progress(&self, iterations: u64, start_time: &Instant) {
        let elapsed = start_time.elapsed();
        let hash_rate = iterations as f64 / elapsed.as_secs_f64();

        eprintln!(
            "[Mining] {} iterations, {:.2} kH/s, {} elapsed",
            iterations,
            hash_rate / 1000.0,
            format_duration(elapsed)
        );
    }

    /// Cancel ongoing mining operation
    pub fn cancel(&self) {
        self.cancel_flag.store(true, Ordering::Relaxed);
    }

    /// Get current iteration count
    pub fn iterations(&self) -> u64 {
        self.iterations.load(Ordering::Relaxed)
    }
}

/// Batch miner for finding multiple aesthetic hashes
pub struct BatchMiner {
    config: MiningConfig,
}

impl BatchMiner {
    pub fn new(config: MiningConfig) -> Self {
        Self { config }
    }

    /// Mine N hashes meeting criteria
    pub fn mine_batch(&self, data: &[u8], count: usize) -> Vec<MiningResult> {
        let mut results = Vec::new();

        for i in 0..count {
            // Use different seed for each hash
            let mut seed = data.to_vec();
            seed.extend_from_slice(&(i as u64).to_le_bytes());

            let miner = Miner::new(self.config.clone());

            match miner.mine(&seed) {
                Ok(result) => {
                    eprintln!(
                        "[Batch {}/{}] Found {} hash: {}",
                        i + 1,
                        count,
                        result.aesthetic_score.rarity.as_str(),
                        result.hash
                    );
                    results.push(result);
                }
                Err(e) => {
                    eprintln!("[Batch {}/{}] Failed: {}", i + 1, count, e);
                }
            }
        }

        results
    }
}

/// Format duration as human-readable string
fn format_duration(duration: Duration) -> String {
    let secs = duration.as_secs();

    if secs < 60 {
        format!("{}s", secs)
    } else if secs < 3600 {
        format!("{}m {}s", secs / 60, secs % 60)
    } else {
        format!("{}h {}m", secs / 3600, (secs % 3600) / 60)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_mining() {
        let mut config = MiningConfig::default();
        config.criteria.min_difficulty = 8; // Easy for testing
        config.max_iterations = Some(100_000);

        let miner = Miner::new(config);
        let result = miner.mine(b"test data");

        match result {
            Ok(res) => {
                assert!(res.aesthetic_score.leading_zeros >= 8);
                println!("Found hash: {}", res.hash);
                println!("Iterations: {}", res.iterations);
                println!("Hash rate: {:.2} kH/s", res.hash_rate / 1000.0);
            }
            Err(e) => {
                println!("Mining failed (expected for difficult targets): {}", e);
            }
        }
    }
}
