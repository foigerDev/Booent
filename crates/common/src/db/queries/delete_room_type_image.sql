DELETE FROM room_type_images
WHERE id = $1
RETURNING id;
