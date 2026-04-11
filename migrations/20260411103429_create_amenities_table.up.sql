CREATE TABLE amenities (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    name TEXT NOT NULL UNIQUE,
    slug TEXT NOT NULL UNIQUE,

    category TEXT NOT NULL,

    icon TEXT,
    display_order INT DEFAULT 0,

    is_active BOOLEAN DEFAULT true,

    created_at TIMESTAMPTZ DEFAULT NOW()
);
