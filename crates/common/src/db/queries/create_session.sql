INSERT INTO sessions (
    id,
    user_id,
    refresh_token_hash,
    user_agent,
    ip_address,
    expires_at,
    revoked
) VALUES (
    $1,  -- id
    $2,  -- user_id
    $3,  -- refresh_token_hash
    $4,  -- user_agent
    $5,  -- ip_address
    $6,  -- expires_at
    $7   -- revoked
)
RETURNING *;
