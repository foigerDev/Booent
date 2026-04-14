ALTER TABLE room_type_images
ADD CONSTRAINT unique_room_image UNIQUE (room_type_id, image_url);
