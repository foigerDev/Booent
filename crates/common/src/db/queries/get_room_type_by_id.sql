SELECT 
    rt.id,
    rt.hotel_id,
    rt.name,
    rt.slug,
    rt.description,
    rt.base_price,
    rt.currency,
    rt.max_adults,
    rt.max_children,
    rt.max_occupancy,
    rt.cover_image_url,
    rt.video_url,
    rt.extra_bed_allowed,
    rt.extra_bed_charge,
    rt.extra_bed_charge_type,
    rt.is_active,
    rt.created_at,
    rt.updated_at,
    COALESCE(
        json_agg(
            json_build_object(
                'bed_type', rtb.bed_type,
                'bed_count', rtb.bed_count
            )
        ) FILTER (WHERE rtb.id IS NOT NULL),
        '[]'
    ) as beds
FROM room_types rt
LEFT JOIN room_type_beds rtb ON rt.id = rtb.room_type_id
WHERE rt.id = $1
GROUP BY rt.id;
