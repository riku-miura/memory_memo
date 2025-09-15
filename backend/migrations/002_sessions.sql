-- Session management table
CREATE TABLE sessions (
    id TEXT PRIMARY KEY, -- session_id from cookie
    user_id TEXT NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
    expires_at DATETIME NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE
);

-- Index for session lookup
CREATE INDEX idx_sessions_id ON sessions(id);
CREATE INDEX idx_sessions_expires ON sessions(expires_at);