use crate::{common_enums, domain_models, errors};
use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct HotelsRow {
    pub id: uuid::Uuid,
    pub slug: String,
    pub name: String,
    pub email: String,
    pub phone: String,
    pub address_line1: String,
    pub address_line2: Option<String>,
    pub city: String,
    pub state: String,
    pub country: String,
    pub pincode: String,
    pub check_in_time: Option<time::Time>,
    pub check_out_time: Option<time::Time>,
    pub logo_url: Option<String>,
    pub cover_image_url: Option<String>,
    pub status: String,
    pub instagram_url: Option<String>,
    pub whatsapp_number: Option<String>,
    pub created_at: time::OffsetDateTime,
    pub updated_at: time::OffsetDateTime,
}

impl HotelsRow {
    pub fn slug_from_name(name: &str) -> String {
        name.to_lowercase()
            .chars()
            .map(|character| {
                if character.is_ascii_alphanumeric() {
                    character
                } else {
                    ' '
                }
            })
            .collect::<String>()
            .split_whitespace()
            .collect::<Vec<_>>()
            .join("-")
    }

    pub fn into_domain_model(
        &self,
    ) -> Result<domain_models::hotels::HotelData, error_stack::Report<errors::HotelErrorTypes>>
    {
        let status = common_enums::HotelStatus::try_from(self.status.as_str())
            .map_err(|_| errors::HotelErrorTypes::InternalServerError)?;
        Ok(domain_models::hotels::HotelData {
            id: self.id,
            slug: self.slug.clone(),
            name: self.name.clone(),
            email: self.email.clone(),
            phone: self.phone.clone(),
            address_line1: self.address_line1.clone(),
            address_line2: self.address_line2.clone(),
            city: self.city.clone(),
            state: self.state.clone(),
            country: self.country.clone(),
            pincode: self.pincode.clone(),
            check_in_time: self.check_in_time,
            check_out_time: self.check_out_time,
            logo_url: self.logo_url.clone(),
            cover_image_url: self.cover_image_url.clone(),
            instagram_url: self.instagram_url.clone(),
            whatsapp_number: self.whatsapp_number.clone(),
            status,
            created_at: self.created_at,
            updated_at: self.updated_at,
        })
    }
}

#[derive(Debug, FromRow)]
pub struct AmenitiesRow {
    pub id: Option<uuid::Uuid>,
    pub name: Option<String>,
    pub slug: Option<String>,
    pub category_id: Option<uuid::Uuid>,
    pub category_name: Option<String>,
    pub category_slug: Option<String>,
    pub icon: Option<String>,
    pub display_order: Option<i32>,
}

impl AmenitiesRow {
    pub fn into_domain_model(self) -> domain_models::hotels::AmenityData {
        domain_models::hotels::AmenityData {
            id: self.id.unwrap_or_default(),
            name: self.name.unwrap_or_default(),
            slug: self.slug.unwrap_or_default(),
            category_id: self.category_id.unwrap_or_default(),
            category_name: self.category_name.unwrap_or_default(),
            category_slug: self.category_slug.unwrap_or_default(),
            icon: self.icon,
            display_order: self.display_order.unwrap_or(0),
        }
    }
}

#[derive(Debug, sqlx::FromRow)]
pub struct RoomTypesRow {
    pub id: uuid::Uuid,
    pub hotel_id: uuid::Uuid,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub base_price: bigdecimal::BigDecimal,
    pub currency: Option<String>,
    pub base_occupancy: i32,
    pub max_adults: i32,
    pub max_children: Option<i32>,
    pub max_occupancy: i32,
    pub is_couple_friendly: bool,
    pub cover_image_url: Option<String>,
    pub video_url: Option<String>,
    pub extra_bed_allowed: Option<bool>,
    pub extra_bed_charge: Option<bigdecimal::BigDecimal>,
    pub extra_bed_charge_type: Option<String>,
    pub is_active: Option<bool>,
    pub created_at: Option<time::OffsetDateTime>,
    pub updated_at: Option<time::OffsetDateTime>,
}

impl RoomTypesRow {
    pub fn into_domain_model(self) -> domain_models::room_types::RoomTypeData {
        domain_models::room_types::RoomTypeData {
            id: self.id,
            hotel_id: self.hotel_id,
            name: self.name,
            slug: self.slug,
            description: self.description,
            base_price: self.base_price,
            base_occupancy: self.base_occupancy,
            max_adults: self.max_adults,
            max_children: self.max_children.unwrap_or(0),
            max_occupancy: self.max_occupancy,
            is_couple_friendly: self.is_couple_friendly,
            cover_image_url: self.cover_image_url,
            video_url: self.video_url,
            extra_bed_allowed: self.extra_bed_allowed.unwrap_or(false),
            extra_bed_charge: self.extra_bed_charge,
            extra_bed_charge_type: self.extra_bed_charge_type,
            is_active: self.is_active.unwrap_or(true),
            created_at: self
                .created_at
                .unwrap_or_else(time::OffsetDateTime::now_utc),
            updated_at: self
                .updated_at
                .unwrap_or_else(time::OffsetDateTime::now_utc),
        }
    }
}
