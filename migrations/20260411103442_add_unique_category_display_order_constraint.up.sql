ALTER TABLE amenities
ADD CONSTRAINT unique_category_display_order
UNIQUE (category_id, display_order);
