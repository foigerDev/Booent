INSERT INTO amenities (name, slug, category_id, display_order)
VALUES

-- BASIC (hotel)
('WiFi', 'wifi', (SELECT id FROM amenity_categories WHERE slug = 'basic'), 1),
('Air Conditioning', 'air_conditioning', (SELECT id FROM amenity_categories WHERE slug = 'basic'), 2),
('Power Backup', 'power_backup', (SELECT id FROM amenity_categories WHERE slug = 'basic'), 3),
('Elevator', 'elevator', (SELECT id FROM amenity_categories WHERE slug = 'basic'), 4),

-- SERVICES
('Room Service', 'room_service', (SELECT id FROM amenity_categories WHERE slug = 'services'), 1),
('Laundry', 'laundry', (SELECT id FROM amenity_categories WHERE slug = 'services'), 2),
('24hr Front Desk', 'front_desk_24hr', (SELECT id FROM amenity_categories WHERE slug = 'services'), 3),
('Housekeeping', 'housekeeping', (SELECT id FROM amenity_categories WHERE slug = 'services'), 4),

-- FOOD & DRINK
('Restaurant', 'restaurant', (SELECT id FROM amenity_categories WHERE slug = 'food_drink'), 1),
('Breakfast Included', 'breakfast_included', (SELECT id FROM amenity_categories WHERE slug = 'food_drink'), 2),
('Bar', 'bar', (SELECT id FROM amenity_categories WHERE slug = 'food_drink'), 3),

-- SAFETY
('CCTV', 'cctv', (SELECT id FROM amenity_categories WHERE slug = 'safety_security'), 1),
('24hr Security', 'security_24hr', (SELECT id FROM amenity_categories WHERE slug = 'safety_security'), 2),

-- WELLNESS
('Swimming Pool', 'swimming_pool', (SELECT id FROM amenity_categories WHERE slug = 'wellness'), 1),
('Gym', 'gym', (SELECT id FROM amenity_categories WHERE slug = 'wellness'), 2),
('Spa', 'spa', (SELECT id FROM amenity_categories WHERE slug = 'wellness'), 3),

-- BUSINESS
('Conference Room', 'conference_room', (SELECT id FROM amenity_categories WHERE slug = 'business'), 1),
('Banquet Hall', 'banquet_hall', (SELECT id FROM amenity_categories WHERE slug = 'business'), 2),

-- ROOM FEATURES
('Television', 'television', (SELECT id FROM amenity_categories WHERE slug = 'room_features'), 1),
('Mini Fridge', 'mini_fridge', (SELECT id FROM amenity_categories WHERE slug = 'room_features'), 2),
('Work Desk', 'work_desk', (SELECT id FROM amenity_categories WHERE slug = 'room_features'), 3),
('Wardrobe', 'wardrobe', (SELECT id FROM amenity_categories WHERE slug = 'room_features'), 4),

-- BATHROOM
('Bathtub', 'bathtub', (SELECT id FROM amenity_categories WHERE slug = 'bathroom'), 1),
('Hot Water', 'hot_water', (SELECT id FROM amenity_categories WHERE slug = 'bathroom'), 2),

-- VIEWS
('Balcony', 'balcony', (SELECT id FROM amenity_categories WHERE slug = 'views'), 1),
('City View', 'city_view', (SELECT id FROM amenity_categories WHERE slug = 'views'), 2),
('Mountain View', 'mountain_view', (SELECT id FROM amenity_categories WHERE slug = 'views'), 3);
