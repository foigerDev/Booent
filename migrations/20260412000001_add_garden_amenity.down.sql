INSERT INTO amenities (name, slug, category_id, display_order)
VALUES (
  'Garden',
  'garden',
  (SELECT id FROM amenity_categories WHERE slug = 'wellness'),
  (
    SELECT COALESCE(MAX(display_order), 0) + 1
    FROM amenities
    WHERE category_id = (SELECT id FROM amenity_categories WHERE slug = 'wellness')
  )
);
