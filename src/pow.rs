use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

pub const MINER_VERSION: u32 = 1;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostDraft {
    pub attachments: Vec<String>,
    pub body: String,
    #[serde(default)]
    pub refs: Vec<String>,
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanonicalParams {
    pub user_pubkey_hex: String,
    pub scope: String, // 't' for thread, 'r' for reply
    pub thread_id: u64,
    pub parent_id: u64,
    pub timestamp_i64: i64,
    pub post_draft: PostDraft,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProofOfWork {
    pub nonce_u64: u64,
    pub miner_version: u32,
    pub timestamp_i64: i64,
}

/// Generate canonical bytes following the HC1 format from haichan
pub fn canonical_bytes_v1(params: &CanonicalParams) -> Vec<u8> {
    let mut bytes = Vec::new();
    
    // Prefix "HC1"
    bytes.extend_from_slice(b"HC1");
    
    // User public key hex (66 bytes for secp256k1)
    bytes.extend_from_slice(params.user_pubkey_hex.as_bytes());
    
    // Scope ('t' or 'r')
    bytes.extend_from_slice(params.scope.as_bytes());
    
    // Thread ID as u64 little endian
    bytes.extend_from_slice(&params.thread_id.to_le_bytes());
    
    // Parent ID as u64 little endian
    bytes.extend_from_slice(&params.parent_id.to_le_bytes());
    
    // Timestamp as i64 little endian
    bytes.extend_from_slice(&params.timestamp_i64.to_le_bytes());
    
    // SHA256 of minified post JSON
    let post_json_minified = minify_post_json(&params.post_draft);
    let post_hash = sha256_bytes(post_json_minified.as_bytes());
    bytes.extend_from_slice(&post_hash);
    
    bytes
}

/// Minify post JSON to ensure deterministic hashing
fn minify_post_json(post: &PostDraft) -> String {
    let mut map = BTreeMap::new();
    map.insert("attachments", serde_json::to_value(&post.attachments).unwrap());
    map.insert("body", serde_json::to_value(&post.body).unwrap());
    map.insert("refs", serde_json::to_value(&post.refs).unwrap());
    map.insert("title", serde_json::to_value(&post.title).unwrap());
    
    serde_json::to_string(&map).unwrap()
}

/// Compute SHA256 hash and return as hex string
pub fn sha256_hex(input: &[u8]) -> String {
    let digest = sha256_bytes(input);
    hex::encode(digest)
}

/// Compute SHA256 hash and return raw bytes
fn sha256_bytes(input: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(input);
    hasher.finalize().into()
}

/// Check if a hex digest starts with the required prefix
pub fn verify_prefix(hex_digest: &str, required_prefix: &str) -> bool {
    hex_digest.starts_with(required_prefix)
}

/// Verify a proof of work solution
pub fn verify_proof_v1(
    params: &CanonicalParams,
    nonce: u64,
    required_prefix: &str,
) -> (bool, String) {
    let mut input = canonical_bytes_v1(params);
    input.extend_from_slice(&nonce.to_le_bytes());
    
    let hash_hex = sha256_hex(&input);
    let valid = verify_prefix(&hash_hex, required_prefix);
    
    (valid, hash_hex)
}

/// Mine for a valid nonce (simple implementation)
pub fn mine_simple(
    canonical_bytes: &[u8],
    required_prefix: &str,
    max_iterations: u64,
) -> Option<(u64, String)> {
    for nonce in 0..max_iterations {
        let mut input = canonical_bytes.to_vec();
        input.extend_from_slice(&nonce.to_le_bytes());
        
        let hash_hex = sha256_hex(&input);
        if verify_prefix(&hash_hex, required_prefix) {
            return Some((nonce, hash_hex));
        }
        
        if nonce % 100_000 == 0 {
            tracing::debug!("Mining progress: {} iterations", nonce);
        }
    }
    None
}

/// Mine personal 21e8 hash for a user's public key
pub fn mine_personal_21e8(pubkey_hex: &str, max_iterations: u64) -> Option<(u64, String)> {
    let mut input_base = Vec::new();
    input_base.extend_from_slice(b"HC1_USER_");
    input_base.extend_from_slice(pubkey_hex.as_bytes());
    input_base.extend_from_slice(b"_PERSONAL");
    
    for nonce in 0..max_iterations {
        let mut input = input_base.clone();
        input.extend_from_slice(&nonce.to_le_bytes());
        
        let hash_hex = sha256_hex(&input);
        if verify_prefix(&hash_hex, "21e8") {
            return Some((nonce, hash_hex));
        }
        
        if nonce % 50_000 == 0 {
            tracing::debug!("Personal 21e8 mining progress for {}: {} iterations", pubkey_hex, nonce);
        }
    }
    None
}

/// Check for extended 21e8 patterns
fn detect_21e8_extension(hash: &str) -> (bool, usize) {
    if !hash.starts_with("21e8") {
        return (false, 0);
    }
    
    let after_21e8 = &hash[4..];
    let mut extension_length = 0;
    
    // Count consecutive zeros after "21e8"
    for ch in after_21e8.chars() {
        if ch == '0' {
            extension_length += 1;
        } else {
            break;
        }
    }
    
    (true, extension_length)
}

/// Calculate PoW difficulty score based on hash
pub fn calculate_pow_difficulty(hash: &str) -> f64 {
    if hash == "pending" || hash.is_empty() {
        return 0.0;
    }
    
    let leading_zeros = hash.chars().take_while(|&c| c == '0').count() as f64;
    let mut score = leading_zeros * 2.0;
    
    let (has_21e8, extension_len) = detect_21e8_extension(hash);
    
    if has_21e8 {
        // Base 21e8 score
        score += 15.0;
        
        // Exponential scoring for extensions: 21e80, 21e800, 21e8000, etc.
        if extension_len > 0 {
            score += (extension_len as f64) * 10.0 + (extension_len as f64).powi(2) * 5.0;
        }
    }
    
    // Additional leading zero bonuses
    if hash.starts_with("0021e8") {
        score += 25.0;
    }
    if hash.starts_with("00021e8") {
        score += 40.0;
    }
    if hash.starts_with("000021e8") {
        score += 60.0;
    }
    
    score
}

/// Get emoji unlock based on 21e8 extension
pub fn get_21e8_emoji(hash: &str) -> Option<&'static str> {
    let (has_21e8, extension_len) = detect_21e8_extension(hash);
    
    if !has_21e8 {
        return None;
    }
    
    match extension_len {
        0 => Some("💎"), // Basic 21e8
        1 => Some("🔮"), // 21e80
        2 => Some("📀"), // 21e800
        3 => Some("🌎"), // 21e8000
        4 => Some("🟢"), // 21e80000
        5 => Some("🎱"), // 21e800000
        6..=usize::MAX => Some("🕳️"), // 21e8000000+
        _ => None,
    }
}

/// Get achievement name for 21e8 extension
pub fn get_21e8_achievement_name(extension_len: usize) -> &'static str {
    match extension_len {
        0 => "Diamond Miner",
        1 => "Crystal Gazer",
        2 => "Digital Pioneer",
        3 => "World Shaper",
        4 => "Emerald Architect",
        5 => "Shadow Master",
        6..=usize::MAX => "Void Walker",
        _ => "Unknown",
    }
}

/// Mine extended 21e8 patterns (21e8, 21e80, 21e800, etc.)
pub fn mine_extended_21e8(
    pubkey_hex: &str,
    target_extension: usize,
    max_iterations: u64,
) -> Option<(u64, String, f64)> {
    let mut input_base = Vec::new();
    input_base.extend_from_slice(b"HC1_USER_");
    input_base.extend_from_slice(pubkey_hex.as_bytes());
    input_base.extend_from_slice(b"_EXTENDED");
    
    for nonce in 0..max_iterations {
        let mut input = input_base.clone();
        input.extend_from_slice(&nonce.to_le_bytes());
        
        let hash_hex = sha256_hex(&input);
        let (has_21e8, extension_len) = detect_21e8_extension(&hash_hex);
        
        if has_21e8 && extension_len >= target_extension {
            let difficulty = calculate_pow_difficulty(&hash_hex);
            return Some((nonce, hash_hex, difficulty));
        }
        
        if nonce % 50_000 == 0 {
            tracing::debug!("Extended 21e8 mining progress for {}: {} iterations", pubkey_hex, nonce);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_canonical_bytes_structure() {
        let post = PostDraft {
            attachments: vec![],
            body: "test body".to_string(),
            refs: vec![],
            title: "test title".to_string(),
        };
        
        let params = CanonicalParams {
            user_pubkey_hex: "03".repeat(33), // 66 char hex string
            scope: "t".to_string(),
            thread_id: 0,
            parent_id: 0,
            timestamp_i64: 1640995200000,
            post_draft: post,
        };
        
        let bytes = canonical_bytes_v1(&params);
        
        // Should start with "HC1"
        assert_eq!(&bytes[0..3], b"HC1");
        
        // Should contain pubkey
        assert_eq!(&bytes[3..69], params.user_pubkey_hex.as_bytes());
        
        // Should contain scope
        assert_eq!(&bytes[69..70], b"t");
    }

    #[test]
    fn test_sha256_hex() {
        let input = b"hello world";
        let hash = sha256_hex(input);
        assert_eq!(hash, "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9");
    }

    #[test]
    fn test_verify_prefix() {
        assert!(verify_prefix("21e8abcd", "21e8"));
        assert!(verify_prefix("0021e8abcd", "0021e8"));
        assert!(!verify_prefix("1e8abcd", "21e8"));
        assert!(!verify_prefix("21e7abcd", "21e8"));
    }

    #[test]
    fn test_post_json_minification() {
        let post = PostDraft {
            attachments: vec!["file1.jpg".to_string()],
            body: "Hello\nWorld!".to_string(),
            refs: vec!["ref1".to_string()],
            title: "My Title".to_string(),
        };
        
        let minified = minify_post_json(&post);
        
        // Should be valid JSON
        let _: serde_json::Value = serde_json::from_str(&minified).unwrap();
        
        // Should contain all keys
        assert!(minified.contains(r#""attachments""#));
        assert!(minified.contains(r#""body""#));
        assert!(minified.contains(r#""refs""#));
        assert!(minified.contains(r#""title""#));
    }
}