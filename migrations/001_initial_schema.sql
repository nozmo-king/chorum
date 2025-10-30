-- Initial schema for haich2 imageboard
CREATE TABLE IF NOT EXISTS boards (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    slug TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    description TEXT,
    is_active BOOLEAN NOT NULL DEFAULT 1,
    thread_count INTEGER NOT NULL DEFAULT 0,
    post_count INTEGER NOT NULL DEFAULT 0,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS threads (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    board_id INTEGER NOT NULL,
    title TEXT NOT NULL,
    content TEXT NOT NULL,
    author_name TEXT,
    author_pubkey TEXT,
    image_path TEXT,
    image_filename TEXT,
    reply_count INTEGER NOT NULL DEFAULT 0,
    is_pinned BOOLEAN NOT NULL DEFAULT 0,
    is_locked BOOLEAN NOT NULL DEFAULT 0,
    bump_score INTEGER NOT NULL DEFAULT 0,
    bumped_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    pow_nonce INTEGER,
    pow_hash TEXT,
    pow_challenge_id TEXT,
    pow_difficulty REAL,
    pow_verified_at DATETIME,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (board_id) REFERENCES boards (id)
);

CREATE TABLE IF NOT EXISTS posts (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    thread_id INTEGER NOT NULL,
    parent_id INTEGER,
    content TEXT NOT NULL,
    author_name TEXT,
    author_pubkey TEXT,
    image_path TEXT,
    image_filename TEXT,
    pow_nonce INTEGER,
    pow_hash TEXT,
    pow_challenge_id TEXT,
    pow_difficulty REAL,
    pow_verified_at DATETIME,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (thread_id) REFERENCES threads (id),
    FOREIGN KEY (parent_id) REFERENCES posts (id)
);

CREATE TABLE IF NOT EXISTS pow_challenges (
    id TEXT PRIMARY KEY,
    user_pubkey_hex TEXT NOT NULL,
    scope TEXT NOT NULL, -- 'thread' or 'reply'
    thread_id INTEGER NOT NULL,
    parent_id INTEGER NOT NULL,
    post_bytes_hash BLOB NOT NULL,
    required_prefix_hex TEXT NOT NULL,
    challenge_version INTEGER NOT NULL DEFAULT 1,
    canonical_bytes BLOB NOT NULL,
    expires_at DATETIME NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS pow_commits (
    id TEXT PRIMARY KEY,
    challenge_id TEXT NOT NULL,
    nonce_u64 INTEGER NOT NULL,
    miner_version INTEGER NOT NULL,
    timestamp_i64 INTEGER NOT NULL,
    solved_hash_hex TEXT NOT NULL,
    thread_id INTEGER,
    post_id INTEGER,
    verified BOOLEAN NOT NULL DEFAULT 0,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (challenge_id) REFERENCES pow_challenges (id),
    FOREIGN KEY (thread_id) REFERENCES threads (id),
    FOREIGN KEY (post_id) REFERENCES posts (id)
);

CREATE TABLE IF NOT EXISTS op_receipts (
    id TEXT PRIMARY KEY,
    operation_type TEXT NOT NULL,
    result_json TEXT NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Create indexes
CREATE INDEX IF NOT EXISTS idx_boards_slug ON boards (slug);
CREATE INDEX IF NOT EXISTS idx_boards_active ON boards (is_active);
CREATE INDEX IF NOT EXISTS idx_threads_board_bumped ON threads (board_id, is_pinned DESC, bumped_at DESC);
CREATE INDEX IF NOT EXISTS idx_posts_thread ON posts (thread_id, created_at);
CREATE INDEX IF NOT EXISTS idx_pow_challenges_expires ON pow_challenges (expires_at);
CREATE INDEX IF NOT EXISTS idx_pow_commits_challenge ON pow_commits (challenge_id);

-- Insert default boards
INSERT OR IGNORE INTO boards (slug, name, description) VALUES
('b', 'Random', 'Random discussion and general topics'),
('tech', 'Technology', 'Technology, programming, and computing'),
('meta', 'Meta', 'Site discussion and feedback');