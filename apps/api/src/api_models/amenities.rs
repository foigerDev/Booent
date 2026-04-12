use common::domain_models::hotels::AmenityData;
use serde::Serialize;

#[derive(Serialize)]
pub struct AmenityCategoryResponse {
    pub id: String,
    pub name: String,
    pub slug: String,
}

#[derive(Serialize)]
pub struct AmenityResponse {
    pub id: String,
    pub name: String,
    pub slug: String,
    pub category: AmenityCategoryResponse,
    pub icon: Option<String>,
    pub display_order: i32,
}

#[derive(Serialize)]
pub struct HotelAmenitiesResponse {
    pub amenities: Vec<AmenityResponse>,
}

impl From<AmenityData> for AmenityResponse {
    fn from(domain: AmenityData) -> Self {
        Self {
            id: domain.id.to_string(),
            name: domain.name,
            slug: domain.slug,
            category: AmenityCategoryResponse {
                id: domain.category_id.to_string(),
                name: domain.category_name,
                slug: domain.category_slug,
            },
            icon: domain.icon,
            display_order: domain.display_order,
        }
    }
}
