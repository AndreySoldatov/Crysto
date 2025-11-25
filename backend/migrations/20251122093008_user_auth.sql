-- User authentication migration
CREATE TABLE users (
    -- Essentials
    id                  BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    username            TEXT NOT NULL UNIQUE,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    -- Password
    auth_hash           TEXT NOT NULL, -- argon hash string contains salt and params

    -- E2EE
    master_key_cipher   TEXT NOT NULL, -- base64
    master_key_nonce    TEXT NOT NULL,  -- base64
    kdf_salt            TEXT NOT NULL
);

CREATE TABLE refresh_tokens (
    id                  BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,

    user_id             BIGINT REFERENCES users(id) ON DELETE CASCADE,

    token_hash          TEXT NOT NULL,
    
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    expires_at          TIMESTAMPTZ NOT NULL DEFAULT (NOW() + INTERVAL '1 month'),
    
    used_at             TIMESTAMPTZ,
    revoked_at          TIMESTAMPTZ
);

CREATE INDEX idx_refresh_token_user_expiry ON refresh_tokens(user_id, expires_at);