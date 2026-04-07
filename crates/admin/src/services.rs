use common::db::hotels_interface::HotelRepository;
use common::common_enums::HotelStatus;
use common::domain_models::hotels::HotelData;
use error_stack::ResultExt;
use sqlx::PgPool;

pub async fn update_hotel_status(
    pool: &PgPool,
    hotel_id: uuid::Uuid,
    status: HotelStatus,
) -> Result<HotelData, error_stack::Report<common::errors::HotelErrorTypes>> {
    pool.update_hotel_status(hotel_id, &status.to_string())
        .await
        .change_context(common::errors::HotelErrorTypes::InternalServerError)
}
