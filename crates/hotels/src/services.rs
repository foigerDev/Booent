use common::db::hotels_interface::HotelRepository;
use common::domain_models::hotels::{self, HotelData, HotelBrandingData, HotelBrandingUpdateRequest};
use common::domain_models::room_types::{BedInfo, RoomTypeCreateRequest, CombinedRoomData};
use common::errors::HotelErrorTypes;
use sqlx::PgPool;
use uuid::Uuid;
use crate::utils::is_hotel_registered;


pub async fn create_hotel(
    pool: &PgPool,
    req: hotels::HotelCreateRequest,
) -> Result<HotelData, error_stack::Report<HotelErrorTypes>> {

    if is_hotel_registered(pool, &req.name, &req.email).await? {
        return Err(error_stack::Report::new(HotelErrorTypes::HotelAlreadyExists));
    }

    let hotel_data = pool.create_hotel(req).await?;

    Ok(hotel_data)
}

pub async fn add_user_to_hotel(
    pool: &PgPool,
    user_id: &str,
    hotel_id: Uuid,
) -> Result<(), error_stack::Report<HotelErrorTypes>> {
    pool.add_user_to_hotel(user_id, hotel_id).await
}

pub async fn update_hotel(
    pool: &PgPool,
    hotel_id: Uuid,
    req: hotels::HotelUpdateRequest,
) -> Result<HotelData, error_stack::Report<HotelErrorTypes>> {
    pool.update_hotel(hotel_id, req).await
}

pub async fn update_hotel_branding(
    pool: &PgPool,
    hotel_id: Uuid,
    req: HotelBrandingUpdateRequest,
) -> Result<HotelBrandingData, error_stack::Report<HotelErrorTypes>> {
    let hotel_data = pool.update_hotel_branding(
        hotel_id,
        req.instagram_url.clone(),
        req.whatsapp_number.clone(),
    ).await?;

    let amenities = if let Some(amenity_ids) = &req.amenity_ids {
        pool.delete_hotel_amenities(hotel_id).await?;
        let amenity_ids_vec: Vec<Uuid> = amenity_ids.clone();
        pool.insert_hotel_amenities(hotel_id, &amenity_ids_vec).await?;
        pool.get_amenities_by_ids(&amenity_ids_vec).await?
    } else {
        vec![]
    };

    Ok(HotelBrandingData { hotel: hotel_data, amenities })
}

pub async fn create_room_type(
    pool: &PgPool,
    hotel_id: Uuid,
    req: RoomTypeCreateRequest,
) -> Result<CombinedRoomData, error_stack::Report<HotelErrorTypes>> {
    let room_type = pool.create_room_type(hotel_id, req.clone()).await?;

    let beds: Vec<BedInfo> = req.beds.iter().map(|b| BedInfo {
        bed_type: b.bed_type.clone(),
        bed_count: b.bed_count,
    }).collect();
    
    pool.insert_room_type_beds(room_type.id, beds.clone()).await?;

    Ok(CombinedRoomData {
        id: room_type.id,
        hotel_id: room_type.hotel_id,
        name: room_type.name,
        slug: room_type.slug,
        description: room_type.description,
        base_price: room_type.base_price,
        max_adults: room_type.max_adults,
        max_children: room_type.max_children,
        max_occupancy: room_type.max_occupancy,
        beds,
        cover_image_url: room_type.cover_image_url,
        video_url: room_type.video_url,
        extra_bed_allowed: room_type.extra_bed_allowed,
        extra_bed_charge: room_type.extra_bed_charge,
        extra_bed_charge_type: room_type.extra_bed_charge_type,
        is_active: room_type.is_active,
        created_at: room_type.created_at,
        updated_at: room_type.updated_at,
    })
}
    