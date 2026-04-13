CREATE TABLE room_amenities (
    room_type_id UUID NOT NULL,
    amenity_id UUID NOT NULL,

    created_at TIMESTAMPTZ DEFAULT NOW(),

    PRIMARY KEY (room_type_id, amenity_id),

    FOREIGN KEY (room_type_id)
        REFERENCES room_types(id)
        ON DELETE CASCADE,

    FOREIGN KEY (amenity_id)
        REFERENCES amenities(id)
        ON DELETE CASCADE
);
