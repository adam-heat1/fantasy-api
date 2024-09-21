use crate::{handlers::ads::response_models::AdResponse, repositories::ads::AdRepository};
use sqlx::Error;

pub struct AdService;

impl AdService {
    pub async fn get_active_ads() -> Result<Vec<AdResponse>, Error> {
        AdRepository::fetch_active_ads().await
    }
}
