SELECT id, room_type_id, image_url, image_type, display_order, created_at
FROM room_type_images
WHERE room_type_id = $1
ORDER BY display_order ASC;
