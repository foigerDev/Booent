use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct RoomTypeData {
    pub id: Uuid,
    pub hotel_id: Uuid,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub base_price: f64,
    pub currency: String,
    pub max_adults: i32,
    pub max_children: i32,
    pub max_occupancy: i32,
    pub bed_type: Option<String>,
    pub bed_count: i32,
    pub cover_image_url: Option<String>,
    pub video_url: Option<String>,
    pub extra_bed_allowed: bool,
    pub extra_bed_charge: Option<f64>,
    pub extra_bed_charge_type: Option<String>,
    pub is_active: bool,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}
