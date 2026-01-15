//! Aesthetic scoring for proof-of-work hashes
//!
//! The 21e8 paradigm: Some hashes are more beautiful than others.
//! This module implements "hashthetics" - aesthetic evaluation of cryptographic hashes.

use crate::Hash256;
use serde::{Deserialize, Serialize};

/// Aesthetic properties detected in a hash
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AestheticScore {
    /// Number of leading zero bits
    pub leading_zeros: u32,

    /// Is the hash a palindrome?
    pub is_palindrome: bool,

    /// Contains the mythic 21e8 sequence?
    pub contains_21e8: bool,

    /// Pattern repetition score (higher = more patterns)
    pub pattern_score: u32,

    /// Total aesthetic supremacy score
    pub total_score: u32,

    /// Rarity class (Common, Uncommon, Rare, Epic, Legendary, Supreme)
    pub rarity: RarityClass,
}

/// Rarity classification for hash aesthetics
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum RarityClass {
    Common,      // 0-99
    Uncommon,    // 100-499
    Rare,        // 500-999
    Epic,        // 1000-4999
    Legendary,   // 5000-9999
    Supreme,     // 10000+
}

impl RarityClass {
    pub fn from_score(score: u32) -> Self {
        match score {
            0..=99 => RarityClass::Common,
            100..=499 => RarityClass::Uncommon,
            500..=999 => RarityClass::Rare,
            1000..=4999 => RarityClass::Epic,
            5000..=9999 => RarityClass::Legendary,
            _ => RarityClass::Supreme,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            RarityClass::Common => "Common",
            RarityClass::Uncommon => "Uncommon",
            RarityClass::Rare => "Rare",
            RarityClass::Epic => "Epic",
            RarityClass::Legendary => "Legendary",
            RarityClass::Supreme => "Supreme",
        }
    }

    pub fn color_code(&self) -> &'static str {
        match self {
            RarityClass::Common => "gray",
            RarityClass::Uncommon => "green",
            RarityClass::Rare => "blue",
            RarityClass::Epic => "purple",
            RarityClass::Legendary => "orange",
            RarityClass::Supreme => "gold",
        }
    }
}

impl AestheticScore {
    /// Compute aesthetic score for a hash
    pub fn evaluate(hash: &Hash256) -> Self {
        let leading_zeros = hash.leading_zeros();
        let is_palindrome = hash.is_palindrome();
        let contains_21e8 = hash.contains_21e8();
        let pattern_score = hash.pattern_score();

        // Scoring formula (balancing rarity and aesthetic appeal)
        let mut total_score = 0u32;

        // Leading zeros are exponentially valuable
        total_score += (leading_zeros as u32) * 100;

        // Palindrome is rare and beautiful
        if is_palindrome {
            total_score += 5000;
        }

        // 21e8 is mythic
        if contains_21e8 {
            total_score += 2108; // The number itself!
        }

        // Pattern repetitions add aesthetic value
        total_score += pattern_score * 10;

        let rarity = RarityClass::from_score(total_score);

        Self {
            leading_zeros,
            is_palindrome,
            contains_21e8,
            pattern_score,
            total_score,
            rarity,
        }
    }

    /// Check if this hash meets minimum aesthetic criteria
    pub fn is_supreme(&self) -> bool {
        self.rarity >= RarityClass::Legendary
    }

    /// Check if this hash is worthy of collection
    pub fn is_collectible(&self) -> bool {
        self.rarity >= RarityClass::Rare
    }

    /// Generate a human-readable description
    pub fn describe(&self) -> String {
        let mut desc = Vec::new();

        if self.leading_zeros > 0 {
            desc.push(format!("{} leading zeros", self.leading_zeros));
        }

        if self.is_palindrome {
            desc.push("palindromic".to_string());
        }

        if self.contains_21e8 {
            desc.push("contains 21e8 sequence".to_string());
        }

        if self.pattern_score > 10 {
            desc.push(format!("{} patterns", self.pattern_score));
        }

        if desc.is_empty() {
            "ordinary".to_string()
        } else {
            desc.join(", ")
        }
    }
}

/// Aesthetic criteria for PoW mining
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AestheticCriteria {
    /// Minimum difficulty (leading zeros)
    pub min_difficulty: u32,

    /// Require palindrome?
    pub require_palindrome: bool,

    /// Require 21e8 sequence?
    pub require_21e8: bool,

    /// Minimum pattern score
    pub min_pattern_score: u32,

    /// Minimum total aesthetic score
    pub min_total_score: u32,
}

impl AestheticCriteria {
    /// Standard difficulty (like Bitcoin)
    pub fn standard() -> Self {
        Self {
            min_difficulty: 20,
            require_palindrome: false,
            require_21e8: false,
            min_pattern_score: 0,
            min_total_score: 0,
        }
    }

    /// Aesthetic supremacy mode (very rare hashes)
    pub fn supreme() -> Self {
        Self {
            min_difficulty: 30,
            require_palindrome: false,
            require_21e8: false,
            min_pattern_score: 15,
            min_total_score: 5000,
        }
    }

    /// The mythic 21e8 mode
    pub fn twentyone_e8() -> Self {
        Self {
            min_difficulty: 25,
            require_palindrome: false,
            require_21e8: true,
            min_pattern_score: 10,
            min_total_score: 3000,
        }
    }

    /// Check if a hash meets these criteria
    pub fn validate(&self, score: &AestheticScore) -> bool {
        if score.leading_zeros < self.min_difficulty {
            return false;
        }

        if self.require_palindrome && !score.is_palindrome {
            return false;
        }

        if self.require_21e8 && !score.contains_21e8 {
            return false;
        }

        if score.pattern_score < self.min_pattern_score {
            return false;
        }

        if score.total_score < self.min_total_score {
            return false;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rarity_classification() {
        assert_eq!(RarityClass::from_score(50), RarityClass::Common);
        assert_eq!(RarityClass::from_score(500), RarityClass::Rare);
        assert_eq!(RarityClass::from_score(10000), RarityClass::Supreme);
    }

    #[test]
    fn test_aesthetic_scoring() {
        let hash = Hash256::zero();
        let score = AestheticScore::evaluate(&hash);

        // All zeros should have max leading zeros
        assert_eq!(score.leading_zeros, 256);
        assert!(score.is_supreme());
    }
}
