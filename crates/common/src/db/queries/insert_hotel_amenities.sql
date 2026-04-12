INSERT INTO hotel_amenities (hotel_id, amenity_id)
SELECT $1, unnest($2::uuid[])
ON CONFLICT (hotel_id, amenity_id) DO NOTHING;
