use common::db::hotels_interface::HotelRepository;
use common::domain_models::hotels::HotelData;
use common::errors::HotelErrorTypes;
use sqlx::PgPool;

pub fn slugify(name: &str) -> String {
    name
        .to_lowercase()
        .chars()
        .map(|character| if character.is_ascii_alphanumeric() { character } else { ' ' })
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join("-")
}

pub async fn find_hotel_by_name_email(
    pool: &PgPool,
    name: &str,
    email: &str,
) -> Result<Option<HotelData>, error_stack::Report<HotelErrorTypes>> {
    pool.find_hotel_by_name_email(name, email).await
}

pub async fn is_hotel_registered(
    pool: &PgPool,
    name: &str,
    email: &str,
) -> Result<bool, error_stack::Report<HotelErrorTypes>> {
    let hotel = find_hotel_by_name_email(pool, name, email).await?;
    Ok(hotel.is_some())
}
