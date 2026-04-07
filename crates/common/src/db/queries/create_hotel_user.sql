INSERT INTO hotel_users (
    user_id,
    hotel_id
) VALUES (
    $1,  -- user_id
    $2   -- hotel_id
)
RETURNING *;
