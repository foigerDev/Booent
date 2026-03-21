-- Create sessions table
CREATE TABLE sessions (
    id TEXT PRIMARY KEY,

    user_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,

    refresh_token_hash TEXT NOT NULL,

    user_agent TEXT,

    ip_address TEXT,

    expires_at TIMESTAMPTZ NOT NULL,

    revoked BOOLEAN NOT NULL DEFAULT FALSE,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Index for refresh token lookup 
CREATE INDEX idx_sessions_refresh_token_hash
ON sessions(refresh_token_hash);

-- Index for user session lookup
CREATE INDEX idx_sessions_user_id
ON sessions(user_id);

-- Index for session expiry cleanup
CREATE INDEX idx_sessions_expires_at
ON sessions(expires_at);