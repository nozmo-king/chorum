use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;
use sha2::Digest;

use crate::{
    config::Config,
    db::{DbPool, PowRepository},
    error::{AppError, Result},
    models::{OpReceipt, PowChallenge, PowCommit},
    pow::{canonical_bytes_v1, verify_proof_v1, CanonicalParams, PostDraft, ProofOfWork},
};

#[derive(Serialize)]
pub struct PowParams {
    pub mode: String,
    pub default_prefix: String,
    pub min_miner_version: u32,
    pub suggested_prefix_by_load: String,
}

#[derive(Deserialize)]
pub struct ThreadBeginRequest {
    pub client_op_id: Uuid,
    pub post_draft: PostDraft,
    pub user_pubkey_hex: String,
    pub timestamp_i64: i64,
}

#[derive(Serialize, Deserialize)]
pub struct ThreadBeginResponse {
    pub challenge_id: String,
    pub required_prefix_hex: String,
    pub challenge_version: u32,
    pub op_id: Uuid,
    pub expires_at: String,
    pub post_bytes_hash: String,
    pub canonical_bytes: String,
}

#[derive(Deserialize)]
pub struct ThreadCommitRequest {
    pub op_id: Uuid,
    pub challenge_id: String,
    pub post_draft: PostDraft,
    pub proof: ProofOfWork,
    pub user_pubkey_hex: String,
}

#[derive(Serialize)]
pub struct ThreadCommitResponse {
    pub thread_id: i64,
}

#[derive(Deserialize)]
pub struct ReplyBeginRequest {
    pub client_op_id: Uuid,
    pub post_draft: PostDraft,
    pub user_pubkey_hex: String,
    pub thread_id: i64,
    pub parent_id: Option<i64>,
    pub timestamp_i64: i64,
}

#[derive(Deserialize)]
pub struct ReplyCommitRequest {
    pub op_id: Uuid,
    pub challenge_id: String,
    pub post_draft: PostDraft,
    pub proof: ProofOfWork,
    pub user_pubkey_hex: String,
    pub thread_id: i64,
    pub parent_id: Option<i64>,
}

#[derive(Serialize)]
pub struct ReplyCommitResponse {
    pub post_id: i64,
}

pub async fn pow_params(State(_pool): State<DbPool>) -> Result<Json<PowParams>> {
    let config = Config::new().unwrap();
    
    Ok(Json(PowParams {
        mode: "vanity_prefix".to_string(),
        default_prefix: config.pow_default_prefix.clone(),
        min_miner_version: 1,
        suggested_prefix_by_load: config.pow_default_prefix,
    }))
}

pub async fn thread_begin(
    State(pool): State<DbPool>,
    Json(req): Json<ThreadBeginRequest>,
) -> Result<Json<ThreadBeginResponse>> {
    // Check for existing operation receipt
    if let Some(receipt) = PowRepository::find_op_receipt(&pool, &req.client_op_id.to_string()).await? {
        let response: ThreadBeginResponse = serde_json::from_str(&receipt.result_json)
            .map_err(|_| AppError::Internal)?;
        return Ok(Json(response));
    }

    // Validate public key format
    if req.user_pubkey_hex.len() != 66 || !req.user_pubkey_hex.starts_with("02") && !req.user_pubkey_hex.starts_with("03") {
        return Err(AppError::InvalidPublicKey);
    }

    let config = Config::new().unwrap();
    
    // Create canonical parameters
    let canonical_params = CanonicalParams {
        user_pubkey_hex: req.user_pubkey_hex.clone(),
        scope: "t".to_string(), // 't' for thread
        thread_id: 0, // New thread
        parent_id: 0,
        timestamp_i64: req.timestamp_i64,
        post_draft: req.post_draft.clone(),
    };

    // Generate canonical bytes
    let canonical_bytes = canonical_bytes_v1(&canonical_params);
    
    // Create post bytes hash
    let post_json = serde_json::to_string(&req.post_draft)?;
    let post_bytes_hash = sha2::Sha256::digest(post_json.as_bytes()).to_vec();

    // Create challenge
    let challenge = PowChallenge::new(
        req.user_pubkey_hex,
        "thread".to_string(),
        0,
        0,
        post_bytes_hash.clone(),
        config.pow_default_prefix.clone(),
        canonical_bytes.clone(),
        config.pow_challenge_ttl_seconds,
    );

    // Store challenge
    PowRepository::create_challenge(&pool, &challenge).await?;

    let response = ThreadBeginResponse {
        challenge_id: challenge.id.clone(),
        required_prefix_hex: challenge.required_prefix_hex,
        challenge_version: 1,
        op_id: req.client_op_id,
        expires_at: challenge.expires_at.to_rfc3339(),
        post_bytes_hash: hex::encode(&post_bytes_hash),
        canonical_bytes: hex::encode(&canonical_bytes),
    };

    // Store operation receipt
    let receipt = OpReceipt::new(
        req.client_op_id.to_string(),
        "thread_begin".to_string(),
        serde_json::to_string(&response)?,
    );
    PowRepository::create_op_receipt(&pool, &receipt).await?;

    Ok(Json(response))
}

pub async fn thread_commit(
    State(pool): State<DbPool>,
    Json(req): Json<ThreadCommitRequest>,
) -> Result<Json<ThreadCommitResponse>> {
    // Find challenge
    let challenge = PowRepository::find_challenge(&pool, &req.challenge_id)
        .await?
        .ok_or(AppError::ChallengeNotFound)?;

    // Check if challenge has expired
    if challenge.is_expired() {
        return Err(AppError::ChallengeExpired);
    }

    // Verify the proof
    let canonical_params = CanonicalParams {
        user_pubkey_hex: req.user_pubkey_hex.clone(),
        scope: "t".to_string(),
        thread_id: 0,
        parent_id: 0,
        timestamp_i64: req.proof.timestamp_i64,
        post_draft: req.post_draft.clone(),
    };

    let (is_valid, solved_hash) = verify_proof_v1(
        &canonical_params,
        req.proof.nonce_u64,
        &challenge.required_prefix_hex,
    );

    if !is_valid {
        return Err(AppError::InvalidProofOfWork);
    }

    // Create thread (assuming board_id = 1 for now)
    let thread_id = crate::db::ThreadRepository::create(
        &pool,
        1, // Default board
        &req.post_draft.title,
        &req.post_draft.body,
        None, // author_name
        Some(&req.user_pubkey_hex),
        Some(req.proof.nonce_u64 as i64),
        Some(&solved_hash),
        Some(&req.challenge_id),
    )
    .await?;

    // Create commit record
    let commit = PowCommit {
        id: Uuid::new_v4().to_string(),
        challenge_id: req.challenge_id,
        nonce_u64: req.proof.nonce_u64 as i64,
        miner_version: req.proof.miner_version as i32,
        timestamp_i64: req.proof.timestamp_i64,
        solved_hash_hex: solved_hash,
        thread_id: Some(thread_id),
        post_id: None,
        verified: true,
        created_at: chrono::Utc::now(),
    };
    PowRepository::create_commit(&pool, &commit).await?;

    Ok(Json(ThreadCommitResponse { thread_id }))
}

pub async fn reply_begin(
    State(pool): State<DbPool>,
    Json(req): Json<ReplyBeginRequest>,
) -> Result<Json<ThreadBeginResponse>> {
    // Similar to thread_begin but with different scope and thread_id
    if let Some(receipt) = PowRepository::find_op_receipt(&pool, &req.client_op_id.to_string()).await? {
        let response: ThreadBeginResponse = serde_json::from_str(&receipt.result_json)
            .map_err(|_| AppError::Internal)?;
        return Ok(Json(response));
    }

    if req.user_pubkey_hex.len() != 66 || !req.user_pubkey_hex.starts_with("02") && !req.user_pubkey_hex.starts_with("03") {
        return Err(AppError::InvalidPublicKey);
    }

    let config = Config::new().unwrap();
    
    let canonical_params = CanonicalParams {
        user_pubkey_hex: req.user_pubkey_hex.clone(),
        scope: "r".to_string(), // 'r' for reply
        thread_id: req.thread_id as u64,
        parent_id: req.parent_id.unwrap_or(0) as u64,
        timestamp_i64: req.timestamp_i64,
        post_draft: req.post_draft.clone(),
    };

    let canonical_bytes = canonical_bytes_v1(&canonical_params);
    let post_json = serde_json::to_string(&req.post_draft)?;
    let post_bytes_hash = sha2::Sha256::digest(post_json.as_bytes()).to_vec();

    let challenge = PowChallenge::new(
        req.user_pubkey_hex,
        "reply".to_string(),
        req.thread_id,
        req.parent_id.unwrap_or(0),
        post_bytes_hash.clone(),
        config.pow_default_prefix.clone(),
        canonical_bytes.clone(),
        config.pow_challenge_ttl_seconds,
    );

    PowRepository::create_challenge(&pool, &challenge).await?;

    let response = ThreadBeginResponse {
        challenge_id: challenge.id.clone(),
        required_prefix_hex: challenge.required_prefix_hex,
        challenge_version: 1,
        op_id: req.client_op_id,
        expires_at: challenge.expires_at.to_rfc3339(),
        post_bytes_hash: hex::encode(&post_bytes_hash),
        canonical_bytes: hex::encode(&canonical_bytes),
    };

    let receipt = OpReceipt::new(
        req.client_op_id.to_string(),
        "reply_begin".to_string(),
        serde_json::to_string(&response)?,
    );
    PowRepository::create_op_receipt(&pool, &receipt).await?;

    Ok(Json(response))
}

pub async fn reply_commit(
    State(pool): State<DbPool>,
    Json(req): Json<ReplyCommitRequest>,
) -> Result<Json<ReplyCommitResponse>> {
    let challenge = PowRepository::find_challenge(&pool, &req.challenge_id)
        .await?
        .ok_or(AppError::ChallengeNotFound)?;

    if challenge.is_expired() {
        return Err(AppError::ChallengeExpired);
    }

    let canonical_params = CanonicalParams {
        user_pubkey_hex: req.user_pubkey_hex.clone(),
        scope: "r".to_string(),
        thread_id: req.thread_id as u64,
        parent_id: req.parent_id.unwrap_or(0) as u64,
        timestamp_i64: req.proof.timestamp_i64,
        post_draft: req.post_draft.clone(),
    };

    let (is_valid, solved_hash) = verify_proof_v1(
        &canonical_params,
        req.proof.nonce_u64,
        &challenge.required_prefix_hex,
    );

    if !is_valid {
        return Err(AppError::InvalidProofOfWork);
    }

    let post_id = crate::db::PostRepository::create(
        &pool,
        req.thread_id,
        req.parent_id,
        &req.post_draft.body,
        None,
        Some(&req.user_pubkey_hex),
        Some(req.proof.nonce_u64 as i64),
        Some(&solved_hash),
        Some(&req.challenge_id),
    )
    .await?;

    let commit = PowCommit {
        id: Uuid::new_v4().to_string(),
        challenge_id: req.challenge_id,
        nonce_u64: req.proof.nonce_u64 as i64,
        miner_version: req.proof.miner_version as i32,
        timestamp_i64: req.proof.timestamp_i64,
        solved_hash_hex: solved_hash,
        thread_id: None,
        post_id: Some(post_id),
        verified: true,
        created_at: chrono::Utc::now(),
    };
    PowRepository::create_commit(&pool, &commit).await?;

    Ok(Json(ReplyCommitResponse { post_id }))
}