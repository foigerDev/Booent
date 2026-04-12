UPDATE hotels SET
    instagram_url = COALESCE($1, instagram_url),
    whatsapp_number = COALESCE($2, whatsapp_number),
    updated_at = NOW()
WHERE id = $3
RETURNING *;
