use secrecy::ExposeSecret;
use serde::{Deserialize, Serialize};
use super::common_models;
use common::domain_models::hotels;


#[derive(Deserialize)]
pub struct HotelCreateRequest {
    pub name: String,
    pub cover_image_url: String,
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


