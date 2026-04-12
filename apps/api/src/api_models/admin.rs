use common::common_enums::HotelStatus;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct UpdateHotelStatusRequest {
    pub hotel_id: uuid::Uuid,
    pub status: HotelStatus,
}

#[derive(Serialize)]
pub struct UpdateHotelStatusResponse {
    pub status: HotelStatus,
}
