use sqlx::Error;

pub struct CrossfitService;

impl CrossfitService {
    pub async fn save_open_scores(_year: i64, _workout: i64) -> Result<(), Error> {
        Ok(())
    }
}
