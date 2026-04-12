UPDATE hotels SET
    name = $1,
    slug = $2,
    email = $3,
    phone = $4,
    address_line1 = $5,
    address_line2 = $6,
    city = $7,
    state = $8,
    country = $9,
    pincode = $10,
    check_in_time = $11,
    check_out_time = $12,
    logo_url = $13,
    cover_image_url = $14,
    updated_at = NOW()
WHERE id = $15
RETURNING *;
