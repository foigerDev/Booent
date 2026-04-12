use once_cell::sync::OnceCell;

#[derive(Default)]
pub struct AppCache {
    pub amenities: OnceCell<Vec<crate::domain_models::hotels::AmenityData>>,
}

impl AppCache {
    pub fn new() -> Self {
        Self::default()
    }
}
