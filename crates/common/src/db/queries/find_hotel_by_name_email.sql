SELECT
    id,
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
    cover_image_url,
    instagram_url,
    whatsapp_number,
    status,
    created_at,
    updated_at
FROM hotels
WHERE name = $1
  AND email = $2;