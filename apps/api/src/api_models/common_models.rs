use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct HotelAddress {
    pub address_line1: String,
    pub address_line2: Option<String>,
    pub city: String,
    pub state: String,
    pub country: String,
    pub pincode: String,
}
