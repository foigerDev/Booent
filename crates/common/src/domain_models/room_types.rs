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
    pub max_adults: i32,
    pub max_children: i32,
    pub max_occupancy: i32,
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
    pub max_adults: i32,
    pub max_children: i32,
    pub max_occupancy: i32,
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
    pub max_adults: i32,
    pub max_children: i32,
    pub max_occupancy: i32,
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
