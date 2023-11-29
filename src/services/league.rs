use crate::repositories::league::LeagueRepository;
use crate::{
    data::models::tournament::Tournament,
    handlers::league::{request_models::CreateLeague, response_models::CreateLeagueResponse},
};
use sqlx::Error;

pub struct LeagueService;

impl LeagueService {
    pub async fn create_league(league: &CreateLeague) -> Result<CreateLeagueResponse, Error> {
        let new_league = Tournament {
            id: 0,
            competition_id: league.competition_id,
            name: league.name.clone(),
            logo: None,
            tournament_type_id: league.tournament_type_id,
            locked_events: 0,
            is_private: league.is_private,
            passcode: league.passcode.clone(),
            commissioner_id: league.user_id,
            entries: None,
            competition: None,
            tournament_type: None,
        };

        let league_id = LeagueRepository::insert_tournament(new_league).await?;

        let created_league = CreateLeagueResponse {
            id: league_id,
            name: league.name.clone(),
            user_id: league.user_id,
            competition_id: league.competition_id,
            tournament_type_id: league.tournament_type_id,
            is_private: league.is_private,
            passcode: league.passcode.clone(),
        };

        Ok(created_league)
    }
}
