use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct HotelUsersRow {
    pub id: uuid::Uuid,
    pub user_id: String,
    pub hotel_id: uuid::Uuid,
}
