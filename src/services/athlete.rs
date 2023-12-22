use crate::handlers::athlete::response_models::CompetitionCompetitorResponse;
use crate::repositories::competitor::CompetitorRepository;
use sqlx::Error;

pub struct AthleteService;

impl AthleteService {
    pub async fn get_competition_competitor(
        competition_id: i64,
        competitor_id: i64,
    ) -> Result<CompetitionCompetitorResponse, Error> {
        CompetitorRepository::fetch_competition_competitor(competition_id, competitor_id).await
    }
}
