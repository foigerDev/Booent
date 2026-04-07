UPDATE sessions
SET refresh_token_hash = $1,
    expires_at = $2,
    updated_at = NOW()
WHERE id = $3
  AND revoked = FALSE
  AND expires_at > NOW()
RETURNING *;
