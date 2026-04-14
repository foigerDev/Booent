-- Active: 1768238314791@@127.0.0.1@5432@booent_db@public
INSERT INTO room_type_images (room_type_id, image_url, image_type, display_order) 
VALUES ($1, $2, $3, $4)
RETURNING id, room_type_id,image_url, image_type, display_order, created_at;

