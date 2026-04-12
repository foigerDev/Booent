use super::common_models;
use common::{common_enums, domain_models::hotels};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct HotelCreateRequest {
    pub name: String,
    pub cover_image_url: Option<String>,
    pub logo_url: Option<String>,
    pub phone: String,
    pub email: String,
    pub address: common_models::HotelAddress,
    pub check_in_time: Option<time::Time>,
    pub check_out_time: Option<time::Time>,
}

impl From<HotelCreateRequest> for hotels::HotelCreateRequest {
    fn from(req: HotelCreateRequest) -> Self {
        Self {
            name: req.name,
            cover_image_url: req.cover_image_url,
            logo_url: req.logo_url,
            phone: req.phone,
            email: req.email,
            address_line1: req.address.address_line1,
            address_line2: req.address.address_line2,
            city: req.address.city,
            state: req.address.state,
            country: req.address.country,
            pincode: req.address.pincode,
            check_in_time: req.check_in_time,
            check_out_time: req.check_out_time,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct HotelCreateResponse {
    pub id: String,
    pub status: common_enums::HotelStatus,
    pub name: String,
    pub cover_image_url: Option<String>,
    pub logo_url: Option<String>,
    pub phone: String,
    pub email: String,
    pub address: common_models::HotelAddress,
    pub check_in_time: Option<time::Time>,
    pub check_out_time: Option<time::Time>,
    pub created_at: time::OffsetDateTime,
    pub updated_at: time::OffsetDateTime,
}

#[derive(Deserialize)]
pub struct HotelUpdateRequest {
    pub name: Option<String>,
    pub cover_image_url: Option<String>,
    pub logo_url: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub address: Option<common_models::HotelAddress>,
    pub check_in_time: Option<time::Time>,
    pub check_out_time: Option<time::Time>,
    pub instagram_url: Option<String>,
    pub whatsapp_number: Option<String>,
    
}

impl From<HotelUpdateRequest> for hotels::HotelUpdateRequest {
    fn from(req: HotelUpdateRequest) -> Self {
        Self {
            name: req.name,
            cover_image_url: req.cover_image_url,
            logo_url: req.logo_url,
            phone: req.phone,
            email: req.email,
            address_line1: req.address.as_ref().map(|a| a.address_line1.clone()),
            address_line2: req.address.as_ref().and_then(|a| a.address_line2.clone()),
            city: req.address.as_ref().map(|a| a.city.clone()),
            state: req.address.as_ref().map(|a| a.state.clone()),
            country: req.address.as_ref().map(|a| a.country.clone()),
            pincode: req.address.as_ref().map(|a| a.pincode.clone()),
            check_in_time: req.check_in_time,
            check_out_time: req.check_out_time,
        }
    }
}

pub type HotelUpdateResponse = HotelCreateResponse;

impl From<hotels::HotelData> for HotelCreateResponse {
    fn from(req: hotels::HotelData) -> Self {
        Self {
            id: req.id.to_string(),
            status: req.status,
            name: req.name,
            cover_image_url: req.cover_image_url,
            logo_url: req.logo_url,
            phone: req.phone,
            email: req.email,
            address: common_models::HotelAddress {
                address_line1: req.address_line1,
                address_line2: req.address_line2,
                city: req.city,
                state: req.state,
                country: req.country,
                pincode: req.pincode,
            },
            check_in_time: req.check_in_time,
            check_out_time: req.check_out_time,
            created_at: req.created_at,
            updated_at: req.updated_at,
        }
    }
}
