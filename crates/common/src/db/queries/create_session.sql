INSERT INTO sessions (
    id,
    user_id,
    refresh_token_hash,
    user_agent,
    ip_address,
    expires_at,
    revoked
)
VALUES (
    $1,
    $2,
    $3,
    $4,
    $5,
    $6,
    $7
)
RETURNING
    id,
    user_id,
    refresh_token_hash,
    user_agent,
    ip_address,
    expires_at,
    revoked,
    created_at,
    updated_at;