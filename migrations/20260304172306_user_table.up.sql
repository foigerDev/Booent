-- Add up migration script here
CREATE TABLE users (
    id TEXT PRIMARY KEY,
    auth_provider VARCHAR(50) NOT NULL,
    auth_provider_user_id TEXT NOT NULL,

    name TEXT NOT NULL,

    email TEXT,
    is_email_verified BOOLEAN NOT NULL DEFAULT FALSE,

    phone TEXT,
    is_phone_verified BOOLEAN NOT NULL DEFAULT FALSE,

    picture_url TEXT,

    status VARCHAR(20) NOT NULL DEFAULT 'active',

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT unique_provider_identity 
        UNIQUE (auth_provider, auth_provider_user_id)
);