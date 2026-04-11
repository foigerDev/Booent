ALTER TABLE amenity_categories
ADD CONSTRAINT unique_slug_type UNIQUE (slug, type);
