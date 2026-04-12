CREATE TABLE room_type_images (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    room_type_id UUID NOT NULL REFERENCES room_types(id) ON DELETE CASCADE,

    image_url TEXT NOT NULL,

    image_type TEXT, -- 'bedroom', 'bathroom', 'view', 'amenity'
    display_order INT NOT NULL,

    created_at TIMESTAMPTZ DEFAULT NOW()
);
