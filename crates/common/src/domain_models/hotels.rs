use crate::common_enums;
use time::{OffsetDateTime, Time};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct HotelCreateRequest {
    pub name: String,
    pub email: String,
    pub phone: String,
    pub address_line1: String,
    pub address_line2: Option<String>,
    pub city: String,
    pub state: String,
    pub country: String,
    pub pincode: String,
    pub check_in_time: Option<Time>,
    pub check_out_time: Option<Time>,

    pub logo_url: Option<String>,
    pub cover_image_url: Option<String>,
}

#[derive(Debug, Clone)]
pub struct HotelUpdateRequest {
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address_line1: Option<String>,
    pub address_line2: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub country: Option<String>,
    pub pincode: Option<String>,
    pub check_in_time: Option<Time>,
    pub check_out_time: Option<Time>,
    pub logo_url: Option<String>,
    pub cover_image_url: Option<String>,
}

#[derive(Debug, Clone)]
pub struct HotelData {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub email: String,
    pub phone: String,
    pub address_line1: String,
    pub address_line2: Option<String>,
    pub city: String,
    pub state: String,
    pub country: String,
    pub pincode: String,
    pub check_in_time: Option<Time>,
    pub check_out_time: Option<Time>,

    // Branding
    pub logo_url: Option<String>,
    pub cover_image_url: Option<String>,
    pub instagram_url: Option<String>,
    pub whatsapp_number: Option<String>,

    pub status: common_enums::HotelStatus,

    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

#[derive(Debug, Clone)]
pub struct AmenityData {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub category_id: Uuid,
    pub category_name: String,
    pub category_slug: String,
    pub icon: Option<String>,
    pub display_order: i32,
}

#[derive(Debug, Clone)]
pub struct HotelBrandingUpdateRequest {
    pub instagram_url: Option<String>,
    pub whatsapp_number: Option<String>,
    pub amenity_ids: Option<Vec<Uuid>>,
}

#[derive(Debug, Clone)]
pub struct HotelBrandingData {
    pub hotel: HotelData,
    pub amenities: Vec<AmenityData>,
}
