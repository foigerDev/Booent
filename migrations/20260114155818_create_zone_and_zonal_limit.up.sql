-- Add migration script here
CREATE TABLE zonal_limit (
    zone_id TEXT PRIMARY KEY,
    max_hotels INT NOT NULL CHECK (max_hotels >= 0),
    active BOOLEAN,
    updated_at DATE,
    created_at DATE NOT NULL,
    CHECK (
        updated_at IS NULL
        OR updated_at >= created_at
    )
);
