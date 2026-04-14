-- Active: 1768238314791@@127.0.0.1@5432@booent_db@public
INSERT INTO room_type_beds (room_type_id, bed_type, bed_count) 
VALUES ($1, $2, $3);
