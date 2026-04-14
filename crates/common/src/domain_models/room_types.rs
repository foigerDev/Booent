use crate::common_enums::RoomImageType;
use bigdecimal::BigDecimal;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, strum_macros::EnumString, strum_macros::Display)]
#[strum(serialize_all = "snake_case")]
pub enum BedType {
    Single,
    Double,
    Queen,
    King,
    Twin,
    SofaBed,
}

#[derive(Debug, Clone)]
pub struct BedInfo {
    pub bed_type: BedType,
    pub bed_count: i32,
}

#[derive(Debug, Clone)]
pub struct RoomTypeData {
    pub id: Uuid,
    pub hotel_id: Uuid,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub base_price: BigDecimal,
    pub base_occupancy: i32,
    pub max_adults: i32,
    pub max_children: i32,
    pub max_occupancy: i32,
    pub is_couple_friendly: bool,
    pub cover_image_url: Option<String>,
    pub video_url: Option<String>,
    pub extra_bed_allowed: bool,
    pub extra_bed_charge: Option<BigDecimal>,
    pub extra_bed_charge_type: Option<String>,
    pub is_active: bool,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

#[derive(Debug, Clone)]
pub struct RoomTypeCreateRequest {
    pub name: String,
    pub description: Option<String>,
    pub base_price: BigDecimal,
    pub currency: Option<String>,
    pub base_occupancy: i32,
    pub max_adults: i32,
    pub max_children: i32,
    pub max_occupancy: i32,
    pub is_couple_friendly: bool,
    pub beds: Vec<BedInfo>,
    pub cover_image_url: Option<String>,
    pub video_url: Option<String>,
    pub extra_bed_allowed: Option<bool>,
    pub extra_bed_charge: Option<BigDecimal>,
    pub extra_bed_charge_type: Option<String>,
}

#[derive(Debug, Clone)]
pub struct CombinedRoomData {
    pub id: Uuid,
    pub hotel_id: Uuid,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub base_price: BigDecimal,
    pub base_occupancy: i32,
    pub max_adults: i32,
    pub max_children: i32,
    pub max_occupancy: i32,
    pub is_couple_friendly: bool,
    pub beds: Vec<BedInfo>,
    pub cover_image_url: Option<String>,
    pub video_url: Option<String>,
    pub extra_bed_allowed: bool,
    pub extra_bed_charge: Option<BigDecimal>,
    pub extra_bed_charge_type: Option<String>,
    pub is_active: bool,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

#[derive(Debug, Clone)]
pub struct RoomTypeImageData {
    pub id: Uuid,
    pub room_type_id: Uuid,
    pub image_url: String,
    pub image_type: Option<RoomImageType>,
    pub display_order: i32,
    pub created_at: OffsetDateTime,
}
