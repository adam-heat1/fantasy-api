use crate::data::models::open_score::OpenScore;
use crate::repositories::open::OpenRepository;
use sqlx::Error;

pub struct OpenService;

impl OpenService {
    pub async fn get_open_scores() -> Result<OpenScore, Error> {
        OpenRepository::fetch_open_scores(3i64).await
    }
}
