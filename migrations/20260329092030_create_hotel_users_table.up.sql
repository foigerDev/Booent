-- Add up migration script here
CREATE TABLE hotel_users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id Text NOT NULL,
    hotel_id UUID NOT NULL,
    CONSTRAINT fk_user
        FOREIGN KEY(user_id)
        REFERENCES users(id)
        ON DELETE CASCADE,

    CONSTRAINT fk_hotel
        FOREIGN KEY(hotel_id)
        REFERENCES hotels(id)
        ON DELETE CASCADE,

    CONSTRAINT unique_user_hotel
        UNIQUE(user_id, hotel_id)
);