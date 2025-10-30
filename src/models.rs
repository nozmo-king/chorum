use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Board {
    pub id: i64,
    pub slug: String,
    pub name: String,
    pub description: Option<String>,
    pub is_active: bool,
    pub thread_count: i32,
    pub post_count: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Thread {
    pub id: i64,
    pub board_id: i64,
    pub title: String,
    pub content: String,
    pub author_name: Option<String>,
    pub author_pubkey: Option<String>,
    pub image_path: Option<String>,
    pub image_filename: Option<String>,
    pub reply_count: i32,
    pub is_pinned: bool,
    pub is_locked: bool,
    pub bump_score: i32,
    pub bumped_at: DateTime<Utc>,
    pub pow_nonce: Option<i64>,
    pub pow_hash: Option<String>,
    pub pow_challenge_id: Option<String>,
    pub pow_difficulty: Option<f64>,
    pub pow_verified_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Post {
    pub id: i64,
    pub thread_id: i64,
    pub parent_id: Option<i64>,
    pub content: String,
    pub author_name: Option<String>,
    pub author_pubkey: Option<String>,
    pub image_path: Option<String>,
    pub image_filename: Option<String>,
    pub pow_nonce: Option<i64>,
    pub pow_hash: Option<String>,
    pub pow_challenge_id: Option<String>,
    pub pow_difficulty: Option<f64>,
    pub pow_verified_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct PowChallenge {
    pub id: String, // UUID
    pub user_pubkey_hex: String,
    pub scope: String, // 'thread' or 'reply'
    pub thread_id: i64,
    pub parent_id: i64,
    pub post_bytes_hash: Vec<u8>,
    pub required_prefix_hex: String,
    pub challenge_version: i32,
    pub canonical_bytes: Vec<u8>,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct PowCommit {
    pub id: String, // UUID
    pub challenge_id: String,
    pub nonce_u64: i64,
    pub miner_version: i32,
    pub timestamp_i64: i64,
    pub solved_hash_hex: String,
    pub thread_id: Option<i64>,
    pub post_id: Option<i64>,
    pub verified: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub pubkey_hex: String,
    pub btc_address: Option<String>,
    pub personal_21e8_hash: Option<String>,
    pub personal_21e8_nonce: Option<i64>,
    pub personal_21e8_achieved_at: Option<DateTime<Utc>>,
    pub post_count: i32,
    pub thread_count: i32,
    pub total_pow_difficulty: f64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct OpReceipt {
    pub id: String, // UUID - matches client_op_id
    pub operation_type: String, // 'thread_begin', 'thread_commit', etc.
    pub result_json: String,
    pub created_at: DateTime<Utc>,
}

impl PowChallenge {
    pub fn new(
        user_pubkey_hex: String,
        scope: String,
        thread_id: i64,
        parent_id: i64,
        post_bytes_hash: Vec<u8>,
        required_prefix_hex: String,
        canonical_bytes: Vec<u8>,
        ttl_seconds: u64,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            user_pubkey_hex,
            scope,
            thread_id,
            parent_id,
            post_bytes_hash,
            required_prefix_hex,
            challenge_version: 1,
            canonical_bytes,
            expires_at: now + chrono::Duration::seconds(ttl_seconds as i64),
            created_at: now,
        }
    }

    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expires_at
    }
}

impl OpReceipt {
    pub fn new(id: String, operation_type: String, result_json: String) -> Self {
        Self {
            id,
            operation_type,
            result_json,
            created_at: Utc::now(),
        }
    }
}