use serde_derive::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate, Clone, Debug)]
pub struct LeagueAthletes {
    #[validate(range(min = 1))]
    #[serde(rename = "competitionId")]
    pub competition_id: u64,
}

#[derive(Deserialize, Validate, Clone, Debug)]
pub struct UserLeaguesRequest {
    #[validate(range(min = 1))]
    #[serde(rename = "userId")]
    pub user_id: i64,
}

#[derive(Deserialize, Validate, Clone, Debug)]
pub struct Pick {
    #[serde(rename = "competitorId")]
    pub competitor_id: i64,
    pub rank: i64,
}

#[derive(Deserialize, Validate, Clone, Debug)]
pub struct CreatePickRequest {
    #[validate(range(min = 1))]
    #[serde(rename = "tournamentUserId")]
    pub tournament_user_id: i64,
    #[serde(rename = "previousPick")]
    pub previous_pick: Pick,
    #[serde(rename = "newPick")]
    pub new_pick: Pick,
}

#[derive(Deserialize, Validate, Clone, Debug)]
pub struct UserLeaguePicksRequest {
    #[validate(range(min = 1))]
    #[serde(rename = "userTournamentId")]
    pub user_tournament_id: i64,
}

#[derive(Deserialize, Validate, Clone, Debug)]
pub struct LeagueLeaderboardRequest {
    #[validate(range(min = 1))]
    #[serde(rename = "tournamentId")]
    pub tournament_id: i64,
}

#[derive(Deserialize, Validate, Clone, Debug)]
pub struct LeaderboardMatchupRequest {
    #[validate(range(min = 1))]
    #[serde(rename = "tournamentId")]
    pub tournament_id: i64,
    #[validate(range(min = 1))]
    #[serde(rename = "userId")]
    pub user_id: i64,
    #[validate(range(min = 0))]
    #[serde(rename = "competitorId")]
    pub competitor_id: i64,
}

#[derive(Deserialize, Validate, Clone, Debug)]
pub struct WorkoutPredictionRequest {
    #[validate(range(min = 1))]
    #[serde(rename = "competitionId")]
    pub competition_id: i64,
    #[validate(range(min = 1))]
    pub ordinal: i64,
}

#[derive(Deserialize, Validate, Clone, Debug)]
pub struct OpenLeague {
    #[validate(range(min = 1))]
    #[serde(rename = "userId")]
    pub user_id: u64,
    #[validate(range(min = 1))]
    #[serde(rename = "competitionId")]
    pub competition_id: u64,
}

#[derive(Deserialize, Validate, Clone, Debug)]
pub struct CreateLeague {
    #[validate(length(min = 3))]
    pub name: String,
    #[validate(range(min = 1))]
    #[serde(rename = "userId")]
    pub user_id: u64,
    #[validate(range(min = 1))]
    #[serde(rename = "competitionId")]
    pub competition_id: u64,
    #[validate(range(min = 1))]
    #[serde(rename = "tournamentTypeId")]
    pub tournament_type_id: u64,
    #[serde(rename = "isPrivate")]
    pub is_private: bool,
    pub passcode: Option<String>,
}

#[derive(Deserialize, Validate, Clone, Debug)]
pub struct JoinLeague {
    #[validate(range(min = 1))]
    #[serde(rename = "userId")]
    pub user_id: i64,
    #[validate(range(min = 1))]
    #[serde(rename = "tournamentId")]
    pub tournament_id: i64,
}

#[derive(Deserialize, Validate, Clone, Debug)]
pub struct CompetitionWorkoutRequest {
    #[validate(range(min = 1))]
    #[serde(rename = "competitionId")]
    pub competition_id: i64,
    #[validate(range(min = 1))]
    pub ordinal: i64,
}
