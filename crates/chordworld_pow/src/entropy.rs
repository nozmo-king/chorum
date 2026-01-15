//! Deterministic entropy pool from aesthetic hashes
//!
//! Beautiful hashes become sources of musical randomness.
//! Each hash is a seed for xenharmonic exploration.

use crate::{AestheticScore, Hash256, MicrotonalScale};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Entry in the entropy pool
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntropyEntry {
    /// The aesthetic hash
    pub hash: Hash256,

    /// Aesthetic score
    pub aesthetic: AestheticScore,

    /// Derived microtonal scale
    pub scale: MicrotonalScale,

    /// Timestamp when mined (for ordering)
    pub mined_at: u64,

    /// User-provided label
    pub label: Option<String>,
}

impl EntropyEntry {
    pub fn new(hash: Hash256, aesthetic: AestheticScore) -> Self {
        let mut scale = MicrotonalScale::from_hash(&hash);
        scale.aesthetic_score = aesthetic.total_score;

        Self {
            hash,
            aesthetic,
            scale,
            mined_at: 0, // Will be set by pool
            label: None,
        }
    }

    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }
}

/// Pool of aesthetic hashes used for deterministic entropy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntropyPool {
    /// Collection of entropy entries, indexed by hash
    entries: HashMap<Hash256, EntropyEntry>,

    /// Ordered list of hashes (by mining time)
    ordered_hashes: Vec<Hash256>,

    /// Counter for assigning timestamps
    next_timestamp: u64,
}

impl EntropyPool {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
            ordered_hashes: Vec::new(),
            next_timestamp: 0,
        }
    }

    /// Add an entry to the pool
    pub fn add(&mut self, mut entry: EntropyEntry) {
        entry.mined_at = self.next_timestamp;
        self.next_timestamp += 1;

        let hash = entry.hash;
        self.entries.insert(hash, entry);
        self.ordered_hashes.push(hash);
    }

    /// Get an entry by hash
    pub fn get(&self, hash: &Hash256) -> Option<&EntropyEntry> {
        self.entries.get(hash)
    }

    /// Get entry by index (mining order)
    pub fn get_by_index(&self, index: usize) -> Option<&EntropyEntry> {
        self.ordered_hashes
            .get(index)
            .and_then(|hash| self.entries.get(hash))
    }

    /// Total number of entries
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Iterator over all entries (in mining order)
    pub fn iter(&self) -> impl Iterator<Item = &EntropyEntry> {
        self.ordered_hashes
            .iter()
            .filter_map(move |hash| self.entries.get(hash))
    }

    /// Get only supreme hashes
    pub fn supreme_hashes(&self) -> Vec<&EntropyEntry> {
        self.iter().filter(|e| e.aesthetic.is_supreme()).collect()
    }

    /// Get collectible hashes
    pub fn collectible_hashes(&self) -> Vec<&EntropyEntry> {
        self.iter()
            .filter(|e| e.aesthetic.is_collectible())
            .collect()
    }

    /// Select a hash deterministically using a seed
    pub fn select_by_seed(&self, seed: u64) -> Option<&EntropyEntry> {
        if self.is_empty() {
            return None;
        }

        let index = (seed as usize) % self.len();
        self.get_by_index(index)
    }

    /// Generate a sequence of entropy bytes from multiple hashes
    pub fn generate_entropy(&self, seed: u64, len: usize) -> Vec<u8> {
        let mut entropy = Vec::with_capacity(len);
        let mut current_seed = seed;

        while entropy.len() < len {
            if let Some(entry) = self.select_by_seed(current_seed) {
                let hash_bytes = entry.hash.as_bytes();

                // Use hash bytes as entropy
                for &byte in hash_bytes {
                    if entropy.len() >= len {
                        break;
                    }
                    entropy.push(byte);
                }
            } else {
                // Pool is empty, generate pseudo-random bytes
                entropy.push((current_seed & 0xFF) as u8);
            }

            current_seed = current_seed.wrapping_add(1);
        }

        entropy.truncate(len);
        entropy
    }

    /// Get statistics about the pool
    pub fn stats(&self) -> PoolStats {
        let mut stats = PoolStats::default();

        for entry in self.iter() {
            stats.total_hashes += 1;

            match entry.aesthetic.rarity {
                crate::RarityClass::Common => stats.common += 1,
                crate::RarityClass::Uncommon => stats.uncommon += 1,
                crate::RarityClass::Rare => stats.rare += 1,
                crate::RarityClass::Epic => stats.epic += 1,
                crate::RarityClass::Legendary => stats.legendary += 1,
                crate::RarityClass::Supreme => stats.supreme += 1,
            }

            if entry.aesthetic.contains_21e8 {
                stats.twentyone_e8_count += 1;
            }

            if entry.aesthetic.is_palindrome {
                stats.palindrome_count += 1;
            }
        }

        stats
    }
}

impl Default for EntropyPool {
    fn default() -> Self {
        Self::new()
    }
}

/// Statistics about an entropy pool
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PoolStats {
    pub total_hashes: usize,
    pub common: usize,
    pub uncommon: usize,
    pub rare: usize,
    pub epic: usize,
    pub legendary: usize,
    pub supreme: usize,
    pub twentyone_e8_count: usize,
    pub palindrome_count: usize,
}

impl PoolStats {
    pub fn display(&self) -> String {
        format!(
            "Total: {} | Common: {} | Uncommon: {} | Rare: {} | Epic: {} | Legendary: {} | Supreme: {} | 21e8: {} | Palindromes: {}",
            self.total_hashes,
            self.common,
            self.uncommon,
            self.rare,
            self.epic,
            self.legendary,
            self.supreme,
            self.twentyone_e8_count,
            self.palindrome_count
        )
    }
}

/// Entropy-driven parameter generator
pub struct EntropyGenerator {
    pool: EntropyPool,
    seed: u64,
}

impl EntropyGenerator {
    pub fn new(pool: EntropyPool, seed: u64) -> Self {
        Self { pool, seed }
    }

    /// Generate a random float in [0, 1) range
    pub fn next_float(&mut self) -> f32 {
        let bytes = self.pool.generate_entropy(self.seed, 4);
        self.seed = self.seed.wrapping_add(1);

        if bytes.len() == 4 {
            let value = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
            (value as f64 / u32::MAX as f64) as f32
        } else {
            0.0
        }
    }

    /// Generate a random float in a range
    pub fn range(&mut self, min: f32, max: f32) -> f32 {
        min + self.next_float() * (max - min)
    }

    /// Generate a random integer in a range
    pub fn range_int(&mut self, min: i32, max: i32) -> i32 {
        min + (self.next_float() * (max - min) as f32) as i32
    }

    /// Select a microtonal scale from the pool
    pub fn select_scale(&mut self) -> Option<MicrotonalScale> {
        self.pool
            .select_by_seed(self.seed)
            .map(|e| e.scale.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entropy_pool() {
        let mut pool = EntropyPool::new();

        let hash = Hash256::zero();
        let aesthetic = AestheticScore::evaluate(&hash);
        let entry = EntropyEntry::new(hash, aesthetic);

        pool.add(entry);

        assert_eq!(pool.len(), 1);
        assert!(pool.get(&hash).is_some());
    }

    #[test]
    fn test_entropy_generation() {
        let mut pool = EntropyPool::new();

        for i in 0..10 {
            let mut bytes = [0u8; 32];
            bytes[0] = i;
            let hash = Hash256(bytes);
            let aesthetic = AestheticScore::evaluate(&hash);
            pool.add(EntropyEntry::new(hash, aesthetic));
        }

        let entropy = pool.generate_entropy(42, 100);
        assert_eq!(entropy.len(), 100);

        // Same seed should produce same entropy
        let entropy2 = pool.generate_entropy(42, 100);
        assert_eq!(entropy, entropy2);
    }
}
