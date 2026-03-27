SELECT
    id,
    auth_provider,
    auth_provider_user_id,
    name,
    email,
    is_email_verified,
    phone,
    is_phone_verified,
    picture_url,
    status,
    created_at,
    updated_at
FROM users
WHERE id = $1;