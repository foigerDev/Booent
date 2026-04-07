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

    pub status: common_enums::HotelStatus,

    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}
