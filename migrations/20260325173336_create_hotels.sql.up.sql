-- Add up migration script here
CREATE TABLE hotels (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL,
    slug TEXT NOT NULL UNIQUE,

    email TEXT NOT NULL,
    phone TEXT NOT NULL,

    address_line1 TEXT NOT NULL,
    address_line2 TEXT,
    city TEXT NOT NULL,
    state TEXT NOT NULL,
    country TEXT NOT NULL,
    pincode TEXT NOT NULL,

    check_in_time TIME NOT NULL,
    check_out_time TIME NOT NULL,
    cover_image_url TEXT NOT NULL,
    logo_url TEXT,

    status TEXT NOT NULL DEFAULT 'active',

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT unique_hotel_identity 
    UNIQUE (name, address_line1, pincode)
);