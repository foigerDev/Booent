CREATE TABLE amenity_categories (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    name TEXT NOT NULL,
    slug TEXT NOT NULL UNIQUE,

    type TEXT NOT NULL,

    display_order INT NOT NULL
);
