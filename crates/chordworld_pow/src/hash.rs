//! Hash types and utilities

use serde::{Deserialize, Serialize};
use std::fmt;

/// 256-bit hash (Blake3 output)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Hash256(pub [u8; 32]);

impl Hash256 {
    pub fn zero() -> Self {
        Self([0u8; 32])
    }

    pub fn from_bytes(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    pub fn to_hex(&self) -> String {
        hex::encode(self.0)
    }

    /// Count leading zero bits
    pub fn leading_zeros(&self) -> u32 {
        let mut count = 0;
        for byte in &self.0 {
            let zeros = byte.leading_zeros();
            count += zeros;
            if zeros < 8 {
                break;
            }
        }
        count
    }

    /// Check if hash is a palindrome (when interpreted as hex)
    pub fn is_palindrome(&self) -> bool {
        let hex = self.to_hex();
        hex == hex.chars().rev().collect::<String>()
    }

    /// Check if hash contains the 21e8 magic sequence
    pub fn contains_21e8(&self) -> bool {
        self.to_hex().contains("21e8")
    }

    /// Count repeating patterns in hex representation
    pub fn pattern_score(&self) -> u32 {
        let hex = self.to_hex();
        let mut score = 0;

        // Look for repeating byte patterns
        for i in 0..hex.len() - 1 {
            if hex.as_bytes()[i] == hex.as_bytes()[i + 1] {
                score += 1;
            }
        }

        score
    }

    /// Extract entropy bytes for musical parameter derivation
    pub fn entropy_slice(&self, offset: usize, len: usize) -> &[u8] {
        let end = (offset + len).min(32);
        &self.0[offset..end]
    }
}

impl fmt::Display for Hash256 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_hex())
    }
}

impl From<[u8; 32]> for Hash256 {
    fn from(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }
}

impl From<blake3::Hash> for Hash256 {
    fn from(hash: blake3::Hash) -> Self {
        Self(*hash.as_bytes())
    }
}

// Hex encoding helper
mod hex {
    pub fn encode(bytes: impl AsRef<[u8]>) -> String {
        bytes.as_ref().iter().map(|b| format!("{:02x}", b)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leading_zeros() {
        let hash = Hash256::zero();
        assert_eq!(hash.leading_zeros(), 256);

        let mut bytes = [0xffu8; 32];
        bytes[0] = 0x00;
        let hash = Hash256(bytes);
        assert_eq!(hash.leading_zeros(), 8);
    }

    #[test]
    fn test_21e8_detection() {
        // Create a hash containing "21e8" in hex
        let mut bytes = [0u8; 32];
        bytes[0] = 0x21;
        bytes[1] = 0xe8;
        let hash = Hash256(bytes);
        assert!(hash.contains_21e8());
    }
}
