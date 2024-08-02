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
    #[serde(rename = "tournamentPositionId")]
    pub tournament_position_id: i64,
}

#[derive(Deserialize, Validate, Clone, Debug)]
pub struct CreateShotCallerPickRequest {
    #[validate(range(min = 1))]
    #[serde(rename = "tournamentUserId")]
    pub tournament_user_id: i64,
    #[serde(rename = "workoutId")]
    pub workout_id: i64,
    #[serde(rename = "competitorId")]
    pub competitor_id: i64,
    #[serde(rename = "tournamentPositionId")]
    pub tournament_position_id: i64,
}

#[derive(Deserialize, Validate, Clone, Debug)]
pub struct CreateTopPickRequest {
    #[validate(range(min = 1))]
    #[serde(rename = "tournamentUserId")]
    pub tournament_user_id: i64,
    pub rank: i64,
    #[serde(rename = "competitorId")]
    pub competitor_id: i64,
    #[serde(rename = "tournamentPositionId")]
    pub tournament_position_id: i64,
}

#[derive(Deserialize, Validate, Clone, Debug)]
pub struct NextPick {
    #[serde(rename = "tournamentPositionId")]
    pub tournament_position_id: Option<i64>,
    #[serde(rename = "nextPickId")]
    pub next_pick_id: Option<i64>,
    pub rank: Option<i64>,
}

#[derive(Deserialize, Validate, Clone, Debug)]
pub struct SwapPickRequest {
    #[validate(range(min = 1))]
    #[serde(rename = "tournamentUserId")]
    pub tournament_user_id: i64,
    #[serde(rename = "previousPickId")]
    pub previous_pick_id: i64,
    #[serde(rename = "nextPick")]
    pub next_pick: NextPick,
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
pub struct AthletePoints {
    #[validate(range(min = 1))]
    #[serde(rename = "athleteId")]
    pub athlete_id: u64,
    #[validate(range(min = 1))]
    pub points: f64,
}

#[derive(Deserialize, Validate, Clone, Debug)]
pub struct DeleteShotCallerPickRequest {
    #[validate(range(min = 1))]
    #[serde(rename = "tournamentUserPickId")]
    pub tournament_user_pick_id: i64,
}

#[derive(Deserialize, Validate, Clone, Debug)]
pub struct DeleteTournamentRequest {
    #[validate(range(min = 1))]
    #[serde(rename = "tournamentId")]
    pub tournament_id: i64,
    #[validate(range(min = 1))]
    #[serde(rename = "userId")]
    pub user_id: i64,
}

#[derive(Deserialize, Validate, Clone, Debug)]
pub struct DeleteTournamentUserRequest {
    #[validate(range(min = 1))]
    #[serde(rename = "tournamentUserId")]
    pub tournament_user_id: i64,
    #[validate(range(min = 1))]
    #[serde(rename = "userId")]
    pub user_id: i64,
}

#[derive(Deserialize, Validate, Clone, Debug)]
pub struct InsertScoresRequest {
    #[validate(range(min = 1))]
    #[serde(rename = "competitionId")]
    pub competition_id: i64,
    #[validate(range(min = 1))]
    pub ordinal: i64,
    pub scores: Vec<AthletePoints>,
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
    #[serde(rename = "pickCount")]
    pub pick_count: Option<i64>,
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
