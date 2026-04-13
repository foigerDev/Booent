use common::cache::AppCache;
use common::db::hotels_interface::HotelRepository;
use common::domain_models::hotels::AmenityData;
use common::errors::HotelErrorTypes;
use sqlx::PgPool;
use std::sync::OnceLock;

static AMENITIES_CACHE: OnceLock<AppCache> = OnceLock::new();

fn get_cache() -> &'static AppCache {
    AMENITIES_CACHE.get_or_init(AppCache::new)
}

pub async fn get_hotel_amenities(
    pool: &PgPool
) -> Result<Vec<AmenityData>, error_stack::Report<HotelErrorTypes>> {
    if let Some(amenities) = get_cache().amenities.get() {
        return Ok(amenities.clone());
    }

    let amenities = pool.get_hotel_amenities().await?;
    let _ = get_cache().amenities.set(amenities.clone());
    
    Ok(amenities)
}

pub async fn get_room_amenities(
    pool: &PgPool
) -> Result<Vec<AmenityData>, error_stack::Report<HotelErrorTypes>> {
    if let Some(amenities) = get_cache().room_amenities.get() {
        return Ok(amenities.clone());
    }

    let amenities = pool.get_room_amenities().await?;
    let _ = get_cache().room_amenities.set(amenities.clone());
    
    Ok(amenities)
}
