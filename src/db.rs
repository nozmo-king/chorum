use crate::{error::Result, models::*};
use chrono::{DateTime, Utc};
use sqlx::{Pool, Sqlite};

pub type DbPool = Pool<Sqlite>;

pub struct BoardRepository;
pub struct ThreadRepository;
pub struct PostRepository;
pub struct PowRepository;

impl BoardRepository {
    pub async fn list_active(pool: &DbPool) -> Result<Vec<Board>> {
        let boards = sqlx::query_as!(
            Board,
            r#"
            SELECT id, slug, name, description, is_active, 
                   thread_count, post_count, created_at, updated_at
            FROM boards 
            WHERE is_active = 1
            ORDER BY name
            "#
        )
        .fetch_all(pool)
        .await?;

        Ok(boards)
    }

    pub async fn find_by_slug(pool: &DbPool, slug: &str) -> Result<Option<Board>> {
        let board = sqlx::query_as!(
            Board,
            r#"
            SELECT id, slug, name, description, is_active,
                   thread_count, post_count, created_at, updated_at
            FROM boards 
            WHERE slug = ? AND is_active = 1
            "#,
            slug
        )
        .fetch_optional(pool)
        .await?;

        Ok(board)
    }
}

impl ThreadRepository {
    pub async fn list_by_board(pool: &DbPool, board_id: i64, limit: i64) -> Result<Vec<Thread>> {
        let threads = sqlx::query_as!(
            Thread,
            r#"
            SELECT id, board_id, title, content, author_name, author_pubkey,
                   image_path, image_filename, reply_count, is_pinned, is_locked,
                   bump_score, bumped_at, pow_nonce, pow_hash, pow_challenge_id,
                   pow_difficulty, pow_verified_at, created_at, updated_at
            FROM threads
            WHERE board_id = ?
            ORDER BY is_pinned DESC, bumped_at DESC
            LIMIT ?
            "#,
            board_id,
            limit
        )
        .fetch_all(pool)
        .await?;

        Ok(threads)
    }

    pub async fn find_by_id(pool: &DbPool, id: i64) -> Result<Option<Thread>> {
        let thread = sqlx::query_as!(
            Thread,
            r#"
            SELECT id, board_id, title, content, author_name, author_pubkey,
                   image_path, image_filename, reply_count, is_pinned, is_locked,
                   bump_score, bumped_at, pow_nonce, pow_hash, pow_challenge_id,
                   pow_difficulty, pow_verified_at, created_at, updated_at
            FROM threads
            WHERE id = ?
            "#,
            id
        )
        .fetch_optional(pool)
        .await?;

        Ok(thread)
    }

    pub async fn create(
        pool: &DbPool,
        board_id: i64,
        title: &str,
        content: &str,
        author_name: Option<&str>,
        author_pubkey: Option<&str>,
        pow_nonce: Option<i64>,
        pow_hash: Option<&str>,
        pow_challenge_id: Option<&str>,
    ) -> Result<i64> {
        let now = Utc::now();
        
        let result = sqlx::query!(
            r#"
            INSERT INTO threads (
                board_id, title, content, author_name, author_pubkey,
                pow_nonce, pow_hash, pow_challenge_id, pow_verified_at,
                bumped_at, created_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
            board_id,
            title,
            content,
            author_name,
            author_pubkey,
            pow_nonce,
            pow_hash,
            pow_challenge_id,
            now,
            now,
            now,
            now
        )
        .execute(pool)
        .await?;

        Ok(result.last_insert_rowid())
    }
}

impl PostRepository {
    pub async fn list_by_thread(pool: &DbPool, thread_id: i64) -> Result<Vec<Post>> {
        let posts = sqlx::query_as!(
            Post,
            r#"
            SELECT id, thread_id, parent_id, content, author_name, author_pubkey,
                   image_path, image_filename, pow_nonce, pow_hash, pow_challenge_id,
                   pow_difficulty, pow_verified_at, created_at, updated_at
            FROM posts
            WHERE thread_id = ?
            ORDER BY created_at ASC
            "#,
            thread_id
        )
        .fetch_all(pool)
        .await?;

        Ok(posts)
    }

    pub async fn create(
        pool: &DbPool,
        thread_id: i64,
        parent_id: Option<i64>,
        content: &str,
        author_name: Option<&str>,
        author_pubkey: Option<&str>,
        pow_nonce: Option<i64>,
        pow_hash: Option<&str>,
        pow_challenge_id: Option<&str>,
    ) -> Result<i64> {
        let now = Utc::now();
        
        let result = sqlx::query!(
            r#"
            INSERT INTO posts (
                thread_id, parent_id, content, author_name, author_pubkey,
                pow_nonce, pow_hash, pow_challenge_id, pow_verified_at,
                created_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
            thread_id,
            parent_id,
            content,
            author_name,
            author_pubkey,
            pow_nonce,
            pow_hash,
            pow_challenge_id,
            now,
            now,
            now
        )
        .execute(pool)
        .await?;

        Ok(result.last_insert_rowid())
    }
}

impl PowRepository {
    pub async fn create_challenge(pool: &DbPool, challenge: &PowChallenge) -> Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO pow_challenges (
                id, user_pubkey_hex, scope, thread_id, parent_id,
                post_bytes_hash, required_prefix_hex, challenge_version,
                canonical_bytes, expires_at, created_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
            challenge.id,
            challenge.user_pubkey_hex,
            challenge.scope,
            challenge.thread_id,
            challenge.parent_id,
            challenge.post_bytes_hash,
            challenge.required_prefix_hex,
            challenge.challenge_version,
            challenge.canonical_bytes,
            challenge.expires_at,
            challenge.created_at
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn find_challenge(pool: &DbPool, id: &str) -> Result<Option<PowChallenge>> {
        let challenge = sqlx::query_as!(
            PowChallenge,
            r#"
            SELECT id, user_pubkey_hex, scope, thread_id, parent_id,
                   post_bytes_hash, required_prefix_hex, challenge_version,
                   canonical_bytes, expires_at, created_at
            FROM pow_challenges
            WHERE id = ?
            "#,
            id
        )
        .fetch_optional(pool)
        .await?;

        Ok(challenge)
    }

    pub async fn create_commit(pool: &DbPool, commit: &PowCommit) -> Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO pow_commits (
                id, challenge_id, nonce_u64, miner_version, timestamp_i64,
                solved_hash_hex, thread_id, post_id, verified, created_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
            commit.id,
            commit.challenge_id,
            commit.nonce_u64,
            commit.miner_version,
            commit.timestamp_i64,
            commit.solved_hash_hex,
            commit.thread_id,
            commit.post_id,
            commit.verified,
            commit.created_at
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn create_op_receipt(pool: &DbPool, receipt: &OpReceipt) -> Result<()> {
        sqlx::query!(
            r#"
            INSERT OR REPLACE INTO op_receipts (
                id, operation_type, result_json, created_at
            ) VALUES (?, ?, ?, ?)
            "#,
            receipt.id,
            receipt.operation_type,
            receipt.result_json,
            receipt.created_at
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn find_op_receipt(pool: &DbPool, id: &str) -> Result<Option<OpReceipt>> {
        let receipt = sqlx::query_as!(
            OpReceipt,
            r#"
            SELECT id, operation_type, result_json, created_at
            FROM op_receipts
            WHERE id = ?
            "#,
            id
        )
        .fetch_optional(pool)
        .await?;

        Ok(receipt)
    }
}