CREATE TABLE room_type_beds (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    room_type_id UUID NOT NULL REFERENCES room_types(id) ON DELETE CASCADE,

    bed_type TEXT NOT NULL,
    bed_count INT NOT NULL CHECK (bed_count > 0),

    created_at TIMESTAMPTZ DEFAULT NOW(),

    CONSTRAINT unique_bed_per_room_type UNIQUE (room_type_id, bed_type)
);
