use crate::{db::models::{hotels, hotel_users}, domain_models, errors};
use async_trait::async_trait;
use error_stack::ResultExt;

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

    async fn update_hotel_status(
        &self,
        hotel_id: uuid::Uuid,
        status: &str,
    ) -> Result<domain_models::hotels::HotelData, error_stack::Report<errors::HotelErrorTypes>>;

    async fn update_hotel(
        &self,
        hotel_id: uuid::Uuid,
        hotel_data: domain_models::hotels::HotelUpdateRequest,
    ) -> Result<domain_models::hotels::HotelData, error_stack::Report<errors::HotelErrorTypes>>;

    async fn get_hotel_amenities(
        &self,
    ) -> Result<Vec<domain_models::hotels::AmenityData>, error_stack::Report<errors::HotelErrorTypes>>;

    async fn update_hotel_branding(
        &self,
        hotel_id: uuid::Uuid,
        instagram_url: Option<String>,
        whatsapp_number: Option<String>,
    ) -> Result<domain_models::hotels::HotelData, error_stack::Report<errors::HotelErrorTypes>>;

    async fn delete_hotel_amenities(
        &self,
        hotel_id: uuid::Uuid,
    ) -> Result<(), error_stack::Report<errors::HotelErrorTypes>>;

    async fn insert_hotel_amenities(
        &self,
        hotel_id: uuid::Uuid,
        amenity_ids: &[uuid::Uuid],
    ) -> Result<(), error_stack::Report<errors::HotelErrorTypes>>;

    async fn get_amenities_by_ids(
        &self,
        amenity_ids: &[uuid::Uuid],
    ) -> Result<Vec<domain_models::hotels::AmenityData>, error_stack::Report<errors::HotelErrorTypes>>;

    async fn check_user_owns_hotel(
        &self,
        user_id: &str,
        hotel_id: uuid::Uuid,
    ) -> Result<bool, error_stack::Report<errors::HotelErrorTypes>>;

    async fn create_room_type(
        &self,
        hotel_id: uuid::Uuid,
        req: domain_models::room_types::RoomTypeCreateRequest,
    ) -> Result<domain_models::room_types::RoomTypeData, error_stack::Report<errors::HotelErrorTypes>>;

    async fn insert_room_type_beds(
        &self,
        room_type_id: uuid::Uuid,
        beds: Vec<domain_models::room_types::BedInfo>,
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

    async fn update_hotel_status(
        &self,
        hotel_id: uuid::Uuid,
        status: &str,
    ) -> Result<domain_models::hotels::HotelData, error_stack::Report<errors::HotelErrorTypes>> {
        let hotel = sqlx::query_file_as!(
            hotels::HotelsRow,
            "src/db/queries/update_hotel_status.sql",
            status,
            hotel_id,
        )
        .fetch_optional(self)
        .await
        .attach_printable("Database error while updating hotel status")
        .change_context(errors::HotelErrorTypes::InternalServerError)?;

        match hotel {
            Some(h) => h.into_domain_model()
                .map_err(|_| error_stack::Report::new(errors::HotelErrorTypes::InternalServerError)),
            None => Err(error_stack::Report::new(errors::HotelErrorTypes::HotelNotFound)),
        }
    }

    async fn update_hotel(
        &self,
        hotel_id: uuid::Uuid,
        hotel_data: domain_models::hotels::HotelUpdateRequest,
    ) -> Result<domain_models::hotels::HotelData, error_stack::Report<errors::HotelErrorTypes>> {
        let hotel = sqlx::query_file_as!(
            hotels::HotelsRow,
            "src/db/queries/update_hotel.sql",
            hotel_data.name,
            hotels::HotelsRow::slug_from_name(hotel_data.name.as_deref().unwrap_or("")),
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
            hotel_id,
        )
        .fetch_optional(self)
        .await
        .attach_printable("Database error while updating hotel")
        .change_context(errors::HotelErrorTypes::InternalServerError)?;

        match hotel {
            Some(h) => h.into_domain_model()
                .map_err(|_| error_stack::Report::new(errors::HotelErrorTypes::InternalServerError)),
            None => Err(error_stack::Report::new(errors::HotelErrorTypes::HotelNotFound)),
        }
    }

    async fn get_hotel_amenities(
        &self,
    ) -> Result<Vec<domain_models::hotels::AmenityData>, error_stack::Report<errors::HotelErrorTypes>> {
        let amenities = sqlx::query_file_as!(
            hotels::AmenitiesRow,
            "src/db/queries/get_hotel_amenities.sql"
        )
        .fetch_all(self)
        .await
        .attach_printable("Database error while fetching hotel amenities")
        .change_context(errors::HotelErrorTypes::InternalServerError)?;

        Ok(amenities.into_iter().map(|row| row.into_domain_model()).collect())
    }

    async fn update_hotel_branding(
        &self,
        hotel_id: uuid::Uuid,
        instagram_url: Option<String>,
        whatsapp_number: Option<String>,
    ) -> Result<domain_models::hotels::HotelData, error_stack::Report<errors::HotelErrorTypes>> {
        let hotel = sqlx::query_file_as!(
            hotels::HotelsRow,
            "src/db/queries/update_hotel_branding.sql",
            instagram_url,
            whatsapp_number,
            hotel_id,
        )
        .fetch_optional(self)
        .await
        .attach_printable("Database error while updating hotel branding")
        .change_context(errors::HotelErrorTypes::InternalServerError)?;

        match hotel {
            Some(h) => h.into_domain_model()
                .map_err(|_| error_stack::Report::new(errors::HotelErrorTypes::InternalServerError)),
            None => Err(error_stack::Report::new(errors::HotelErrorTypes::HotelNotFound)),
        }
    }

    async fn delete_hotel_amenities(
        &self,
        hotel_id: uuid::Uuid,
    ) -> Result<(), error_stack::Report<errors::HotelErrorTypes>> {
        sqlx::query_file!("src/db/queries/delete_hotel_amenities.sql", hotel_id)
            .execute(self)
            .await
            .attach_printable("Database error while deleting hotel amenities")
            .change_context(errors::HotelErrorTypes::InternalServerError)?;
        Ok(())
    }

    async fn insert_hotel_amenities(
        &self,
        hotel_id: uuid::Uuid,
        amenity_ids: &[uuid::Uuid],
    ) -> Result<(), error_stack::Report<errors::HotelErrorTypes>> {
        if amenity_ids.is_empty() {
            return Ok(());
        }
        sqlx::query_file!("src/db/queries/insert_hotel_amenities.sql", hotel_id, amenity_ids)
            .execute(self)
            .await
            .attach_printable("Database error while inserting hotel amenities")
            .change_context(errors::HotelErrorTypes::InternalServerError)?;
        Ok(())
    }

    async fn get_amenities_by_ids(
        &self,
        amenity_ids: &[uuid::Uuid],
    ) -> Result<Vec<domain_models::hotels::AmenityData>, error_stack::Report<errors::HotelErrorTypes>> {
        if amenity_ids.is_empty() {
            return Ok(vec![]);
        }
        let amenities = sqlx::query_file_as!(
            hotels::AmenitiesRow,
            "src/db/queries/get_amenities_by_ids.sql",
            amenity_ids
        )
        .fetch_all(self)
        .await
        .attach_printable("Database error while fetching amenities by ids")
        .change_context(errors::HotelErrorTypes::InternalServerError)?;
        Ok(amenities.into_iter().map(|row| row.into_domain_model()).collect())
    }

    async fn check_user_owns_hotel(
        &self,
        user_id: &str,
        hotel_id: uuid::Uuid,
    ) -> Result<bool, error_stack::Report<errors::HotelErrorTypes>> {
        let result = sqlx::query_file!(
            "src/db/queries/check_user_owns_hotel.sql",
            user_id,
            hotel_id
        )
        .fetch_optional(self)
        .await
        .attach_printable("Database error while checking user hotel ownership")
        .change_context(errors::HotelErrorTypes::InternalServerError)?;

        Ok(result.is_some())
    }

    async fn create_room_type(
        &self,
        hotel_id: uuid::Uuid,
        req: domain_models::room_types::RoomTypeCreateRequest,
    ) -> Result<domain_models::room_types::RoomTypeData, error_stack::Report<errors::HotelErrorTypes>> {
        let slug = slugify(&req.name);
        let room_type = sqlx::query_file_as!(
            hotels::RoomTypesRow,
            "src/db/queries/create_room_type.sql",
            hotel_id,
            req.name,
            slug,
            req.description,
            req.base_price,
            req.currency.as_deref().unwrap_or("INR"),
            req.max_adults,
            req.max_children,
            req.max_occupancy,
            req.cover_image_url,
            req.video_url,
            req.extra_bed_allowed,
            req.extra_bed_charge,
            req.extra_bed_charge_type,
        )
        .fetch_one(self)
        .await
        .attach_printable("Database error while creating room type")
        .change_context(errors::HotelErrorTypes::InternalServerError)?;

        Ok(room_type.into_domain_model())
    }

    async fn insert_room_type_beds(
        &self,
        room_type_id: uuid::Uuid,
        beds: Vec<domain_models::room_types::BedInfo>,
    ) -> Result<(), error_stack::Report<errors::HotelErrorTypes>> {
        if beds.is_empty() {
            return Ok(());
        }
        let bed_types: Vec<String> = beds.iter().map(|b| b.bed_type.to_string()).collect();
        let bed_counts: Vec<i32> = beds.iter().map(|b| b.bed_count).collect();
        
        let query = "INSERT INTO room_type_beds (room_type_id, bed_type, bed_count) SELECT $1, UNNEST($2::text[]), UNNEST($3::int[])";
        
        sqlx::query(query)
            .bind(room_type_id)
            .bind(&bed_types[..])
            .bind(&bed_counts[..])
            .execute(self)
            .await
            .attach_printable("Database error while inserting room type beds")
            .change_context(errors::HotelErrorTypes::InternalServerError)?;
        Ok(())
    }
}
