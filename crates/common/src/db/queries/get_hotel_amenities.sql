SELECT 
    a.id,
    a.name,
    a.slug,
    a.category_id,
    c.name as category_name,
    c.slug as category_slug,
    a.icon,
    a.display_order
FROM amenities a
JOIN amenity_categories c ON a.category_id = c.id
WHERE c.type = 'hotel'
ORDER BY c.display_order ASC, a.display_order ASC;
