-- Add user system with personal 21e8 hash generation
CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    pubkey_hex TEXT NOT NULL UNIQUE,
    btc_address TEXT,
    personal_21e8_hash TEXT,
    personal_21e8_nonce INTEGER,
    personal_21e8_achieved_at DATETIME,
    post_count INTEGER NOT NULL DEFAULT 0,
    thread_count INTEGER NOT NULL DEFAULT 0,
    total_pow_difficulty REAL NOT NULL DEFAULT 0,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Update threads to reference user
ALTER TABLE threads ADD COLUMN user_id INTEGER REFERENCES users(id);

-- Update posts to reference user  
ALTER TABLE posts ADD COLUMN user_id INTEGER REFERENCES users(id);

-- Create index on personal 21e8 hash
CREATE INDEX IF NOT EXISTS idx_users_personal_21e8 ON users (personal_21e8_hash);
CREATE INDEX IF NOT EXISTS idx_users_pubkey ON users (pubkey_hex);