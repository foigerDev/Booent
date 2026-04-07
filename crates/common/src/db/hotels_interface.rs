use crate::{db::models::{hotels, hotel_users}, domain_models, errors};
use async_trait::async_trait;
use error_stack::ResultExt;
use time::Time;

fn slugify(name: &str) -> String {
    name
        .to_lowercase()
        .chars()
        .map(|character| if character.is_ascii_alphanumeric() { character } else { ' ' })
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join("-")
}

#[async_trait]
pub trait HotelRepository {
    async fn create_hotel(
        &self,
        hotel_data: domain_models::hotels::HotelCreateRequest,
    ) -> Result<domain_models::hotels::HotelData, error_stack::Report<errors::HotelErrorTypes>>;

    async fn find_hotel_by_name_email(
        &self,
        name: &str,
        email: &str,
    ) -> Result<Option<domain_models::hotels::HotelData>, error_stack::Report<errors::HotelErrorTypes>>;

    async fn add_user_to_hotel(
        &self,
        user_id: &str,
        hotel_id: uuid::Uuid,
    ) -> Result<(), error_stack::Report<errors::HotelErrorTypes>>;

}

#[async_trait]
impl HotelRepository for sqlx::PgPool {
    async fn create_hotel(
        &self,
        hotel_data: domain_models::hotels::HotelCreateRequest,
    ) -> Result<domain_models::hotels::HotelData, error_stack::Report<errors::HotelErrorTypes>> {
        let slug = slugify(&hotel_data.name);
        let hotels = sqlx::query_file_as!(
            hotels::HotelsRow,
            "src/db/queries/create_hotel.sql",
            hotel_data.name,
            slug,
            hotel_data.email,
            hotel_data.phone,
            hotel_data.address_line1,
            hotel_data.address_line2,
            hotel_data.city,
            hotel_data.state,
            hotel_data.country,
            hotel_data.pincode,
            hotel_data.check_in_time,
            hotel_data.check_out_time,
            hotel_data.logo_url,
            hotel_data.cover_image_url,
        )
        .fetch_one(self)
        .await
        .attach_printable("Database error while creating hotel")
        .change_context(errors::HotelErrorTypes::InternalServerError)?;

        let hotels_output = hotels.into_domain_model()
            .map_err(|_|error_stack::Report::new(errors::HotelErrorTypes::InternalServerError))?;

        Ok(hotels_output)
    }

    async fn find_hotel_by_name_email(
        &self,
        name: &str,
        email: &str,
    ) -> Result<Option<domain_models::hotels::HotelData>, error_stack::Report<errors::HotelErrorTypes>> {
        let hotel = sqlx::query_file_as!(
            hotels::HotelsRow,
            "src/db/queries/find_hotel_by_name_email.sql",
            name,
            email
        )
        .fetch_optional(self)
        .await
        .attach_printable("Database error while fetching hotel by name and email")
        .change_context(errors::HotelErrorTypes::InternalServerError)?;

        let hotel_data = hotel
            .map(|hotel_row| hotel_row.into_domain_model())
            .transpose()
            .change_context(errors::HotelErrorTypes::InternalServerError)?;

        Ok(hotel_data)
    }

    async fn add_user_to_hotel(
        &self,
        user_id: &str,
        hotel_id: uuid::Uuid,
    ) -> Result<(), error_stack::Report<errors::HotelErrorTypes>> {
        let _ = sqlx::query_file_as!(
            hotel_users::HotelUsersRow,
            "src/db/queries/create_hotel_user.sql",
            user_id,
            hotel_id
        )
        .fetch_one(self)
        .await
        .attach_printable("Database error while adding user to hotel")
        .change_context(errors::HotelErrorTypes::InternalServerError)?;

        Ok(())
    }
}
