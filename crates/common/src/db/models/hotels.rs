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
    pub cover_image_url: String,
    pub status: String,
    pub created_at: time::OffsetDateTime,
    pub updated_at: time::OffsetDateTime,
}

impl HotelsRow {
    pub fn into_domain_model(
        &self,
    ) -> Result<domain_models::hotels::HotelData, error_stack::Report<errors::HotelErrorTypes>>
    {
        let status = common_enums::HotelStatus::try_from(self.status.as_str())
            .map_err(|_| errors::HotelErrorTypes::InternalServerError)?;
        Ok(domain_models::hotels::HotelData {
            id: self.id.clone(),
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
            status,
            created_at: self.created_at,
            updated_at: self.updated_at,
        })
    }
}
