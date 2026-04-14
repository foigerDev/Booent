use super::common_models;
use common::{
    common_enums,
    domain_models::{hotels, room_types},
};
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

#[derive(Deserialize)]
pub struct HotelBrandingUpdateRequest {
    pub instagram_url: Option<String>,
    pub whatsapp_number: Option<String>,
    pub amenity_ids: Option<Vec<String>>,
}

impl From<HotelBrandingUpdateRequest> for hotels::HotelBrandingUpdateRequest {
    fn from(req: HotelBrandingUpdateRequest) -> Self {
        Self {
            instagram_url: req.instagram_url,
            whatsapp_number: req.whatsapp_number,
            amenity_ids: req.amenity_ids.map(|ids| {
                ids.into_iter()
                    .filter_map(|id| uuid::Uuid::parse_str(&id).ok())
                    .collect()
            }),
        }
    }
}

#[derive(Serialize)]
pub struct AmenityResponse {
    pub id: String,
    pub name: String,
    pub slug: String,
    pub category: AmenityCategoryResponse,
    pub icon: Option<String>,
    pub display_order: i32,
}

#[derive(Serialize)]
pub struct AmenityCategoryResponse {
    pub id: String,
    pub name: String,
    pub slug: String,
}

impl From<hotels::AmenityData> for AmenityResponse {
    fn from(domain: hotels::AmenityData) -> Self {
        Self {
            id: domain.id.to_string(),
            name: domain.name,
            slug: domain.slug,
            category: AmenityCategoryResponse {
                id: domain.category_id.to_string(),
                name: domain.category_name,
                slug: domain.category_slug,
            },
            icon: domain.icon,
            display_order: domain.display_order,
        }
    }
}

#[derive(Serialize)]
pub struct HotelBrandingUpdateResponse {
    pub hotel: HotelCreateResponse,
    pub amenities: Vec<AmenityResponse>,
}

impl From<hotels::HotelBrandingData> for HotelBrandingUpdateResponse {
    fn from(domain: hotels::HotelBrandingData) -> Self {
        Self {
            hotel: HotelCreateResponse::from(domain.hotel),
            amenities: domain
                .amenities
                .into_iter()
                .map(AmenityResponse::from)
                .collect(),
        }
    }
}

#[derive(Deserialize)]
pub struct BedRequest {
    pub bed_type: String,
    pub bed_count: i32,
}

#[derive(Deserialize)]
pub struct RoomTypeCreateRequest {
    pub name: String,
    pub description: Option<String>,
    pub base_price: bigdecimal::BigDecimal,
    pub currency: Option<String>,
    pub base_occupancy: i32,
    pub max_adults: i32,
    pub max_children: i32,
    pub max_occupancy: i32,
    pub is_couple_friendly: bool,
    pub beds: Vec<BedRequest>,
    pub cover_image_url: Option<String>,
    pub video_url: Option<String>,
    pub extra_bed_allowed: Option<bool>,
    pub extra_bed_charge: Option<bigdecimal::BigDecimal>,
    pub extra_bed_charge_type: Option<String>,
}

impl From<RoomTypeCreateRequest> for room_types::RoomTypeCreateRequest {
    fn from(req: RoomTypeCreateRequest) -> Self {
        Self {
            name: req.name,
            description: req.description,
            base_price: req.base_price,
            currency: req.currency,
            base_occupancy: req.base_occupancy,
            max_adults: req.max_adults,
            max_children: req.max_children,
            max_occupancy: req.max_occupancy,
            is_couple_friendly: req.is_couple_friendly,
            beds: req
                .beds
                .into_iter()
                .map(|b| room_types::BedInfo {
                    bed_type: b.bed_type.parse().unwrap_or(room_types::BedType::Single),
                    bed_count: b.bed_count,
                })
                .collect(),
            cover_image_url: req.cover_image_url,
            video_url: req.video_url,
            extra_bed_allowed: req.extra_bed_allowed,
            extra_bed_charge: req.extra_bed_charge,
            extra_bed_charge_type: req.extra_bed_charge_type,
        }
    }
}

#[derive(Serialize)]
pub struct BedResponse {
    pub bed_type: String,
    pub bed_count: i32,
}

impl From<room_types::BedInfo> for BedResponse {
    fn from(bed: room_types::BedInfo) -> Self {
        Self {
            bed_type: bed.bed_type.to_string(),
            bed_count: bed.bed_count,
        }
    }
}

#[derive(Serialize)]
pub struct RoomTypeResponse {
    pub id: String,
    pub hotel_id: String,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub base_price: bigdecimal::BigDecimal,
    pub currency: String,
    pub base_occupancy: i32,
    pub max_adults: i32,
    pub max_children: i32,
    pub max_occupancy: i32,
    pub is_couple_friendly: bool,
    pub beds: Vec<BedResponse>,
    pub cover_image_url: Option<String>,
    pub video_url: Option<String>,
    pub extra_bed_allowed: bool,
    pub extra_bed_charge: Option<bigdecimal::BigDecimal>,
    pub extra_bed_charge_type: Option<String>,
    pub is_active: bool,
    pub created_at: time::OffsetDateTime,
    pub updated_at: time::OffsetDateTime,
}

impl From<room_types::CombinedRoomData> for RoomTypeResponse {
    fn from(room: room_types::CombinedRoomData) -> Self {
        Self {
            id: room.id.to_string(),
            hotel_id: room.hotel_id.to_string(),
            name: room.name,
            slug: room.slug,
            description: room.description,
            base_price: room.base_price,
            currency: "INR".to_string(),
            base_occupancy: room.base_occupancy,
            max_adults: room.max_adults,
            max_children: room.max_children,
            max_occupancy: room.max_occupancy,
            is_couple_friendly: room.is_couple_friendly,
            beds: room.beds.into_iter().map(BedResponse::from).collect(),
            cover_image_url: room.cover_image_url,
            video_url: room.video_url,
            extra_bed_allowed: room.extra_bed_allowed,
            extra_bed_charge: room.extra_bed_charge,
            extra_bed_charge_type: room.extra_bed_charge_type,
            is_active: room.is_active,
            created_at: room.created_at,
            updated_at: room.updated_at,
        }
    }
}

#[derive(Deserialize)]
pub struct RoomTypeAmenitiesUpdateRequest {
    pub amenity_ids: Vec<String>,
}

#[derive(Serialize)]
pub struct RoomTypeAmenitiesUpdateResponse {
    pub room_type_id: String,
    pub amenity_ids: Vec<String>,
}

#[derive(Deserialize, Serialize)]
pub struct RoomTypeImagesUpdateRequest {
    pub image_url: String,
    pub image_type: common::common_enums::RoomImageType,
    pub display_order: i32,
}

#[derive(Serialize)]
pub struct RoomTypeImageResponse {
    pub id: String,
    pub room_type_id: String,
    pub image_url: String,
    pub image_type: Option<common::common_enums::RoomImageType>,
    pub display_order: i32,
    pub created_at: time::OffsetDateTime,
}

impl From<room_types::RoomTypeImageData> for RoomTypeImageResponse {
    fn from(img: room_types::RoomTypeImageData) -> Self {
        Self {
            id: img.id.to_string(),
            room_type_id: img.room_type_id.to_string(),
            image_url: img.image_url,
            image_type: img.image_type,
            display_order: img.display_order,
            created_at: img.created_at,
        }
    }
}
