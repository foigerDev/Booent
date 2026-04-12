SELECT hotel_id FROM hotel_users WHERE user_id = $1 AND hotel_id = $2 LIMIT 1;
