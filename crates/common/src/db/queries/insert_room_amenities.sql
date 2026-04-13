INSERT INTO room_amenities (room_type_id, amenity_id) SELECT $1, UNNEST($2::uuid[]);
