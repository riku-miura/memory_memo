-- Memory Memo Database Schema
-- Initial migration: User accounts, Forever Memos, and Flush Memos

-- Users table
CREATE TABLE users (
    id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
    username TEXT UNIQUE NOT NULL CHECK(length(username) >= 3 AND length(username) <= 50),
    password_hash TEXT NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL
);

-- Forever memos table (persistent until manually deleted)
CREATE TABLE forever_memos (
    id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
    user_id TEXT NOT NULL,
    content TEXT NOT NULL CHECK(length(content) > 0),
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE
);

-- Flush memos table (expire after 24 hours)
CREATE TABLE flush_memos (
    id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
    user_id TEXT NOT NULL,
    content TEXT NOT NULL CHECK(length(content) > 0),
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
    expires_at DATETIME NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE
);

-- Performance indexes
CREATE INDEX idx_forever_memos_user_created ON forever_memos(user_id, created_at DESC);
CREATE INDEX idx_flush_memos_user_created ON flush_memos(user_id, created_at DESC);
CREATE INDEX idx_flush_memos_expires ON flush_memos(expires_at);

-- Unique index on username for fast lookup
CREATE UNIQUE INDEX idx_users_username ON users(username);