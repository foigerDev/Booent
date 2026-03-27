INSERT INTO hotels (
    name,
    slug,
    email,
    phone,
    address_line1,
    address_line2,
    city,
    state,
    country,
    pincode,
    check_in_time,
    check_out_time,
    logo_url,
    cover_image_url
) VALUES (
    $1,  -- name
    $2,  -- slug
    $3,  -- email
    $4,  -- phone
    $5,  -- address_line1
    $6,  -- address_line2
    $7,  -- city
    $8,  -- state
    $9,  -- country (e.g. 'India')
    $10, -- pincode
    $11, -- check_in_time (TIME)
    $12, -- check_out_time (TIME)
    $13, -- logo_url
    $14  -- cover_image_url
)
RETURNING *;