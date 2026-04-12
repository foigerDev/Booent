INSERT INTO room_types (
    hotel_id,
    name,
    slug,
    description,
    base_price,
    currency,
    max_adults,
    max_children,
    max_occupancy,
    cover_image_url,
    video_url,
    extra_bed_allowed,
    extra_bed_charge,
    extra_bed_charge_type
) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
RETURNING *;
