use common::db::hotels_interface::HotelRepository;
use common::domain_models::hotels::{self, HotelData};
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
    