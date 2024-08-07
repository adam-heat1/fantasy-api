use crate::data::models::workout::Workout;
use crate::handlers::props::response_models::PropMatchupDetail;
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CreateLeagueResponse {
    pub id: u64,
    pub name: String,
    #[serde(rename = "userId")]
    pub user_id: u64,
    #[serde(rename = "competitionId")]
    pub competition_id: u64,
    #[serde(rename = "tournamentTypeId")]
    pub tournament_type_id: u64,
    #[serde(rename = "isPrivate")]
    pub is_private: bool,
    pub passcode: Option<String>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct OpenLeagueResponse {
    pub id: u64,
    pub name: String,
    pub logo: Option<String>,
    #[serde(rename = "competitionId")]
    pub competition_id: u64,
    #[serde(rename = "tournamentTypeId")]
    pub tournament_type_id: u64,
    #[serde(rename = "isPrivate")]
    pub is_private: bool,
    pub passcode: Option<String>,
    pub entries: u64,
    #[serde(rename = "pickCount")]
    pub pick_count: u64,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct LeaderboardEntry {
    #[serde(rename = "tournamentUserId")]
    pub tournament_user_id: u64,
    #[serde(rename = "displayName")]
    pub display_name: String,
    pub avatar: String,
    pub points: f64,
    pub event_wins: i64,
    pub ordinal: i64,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct LeaderboardResponse {
    pub tournament: String,
    pub competition: String,
    pub logo: String,
    #[serde(rename = "lockedEvents")]
    pub locked_events: u64,
    pub leaderboard: Vec<LeaderboardEntry>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct MatchupPick {
    #[serde(rename = "competitorId")]
    pub competitor_id: u64,
    #[serde(rename = "predictedRank")]
    pub predicted_rank: u64,
    pub rank: u64,
    #[serde(rename = "firstName")]
    pub first_name: String,
    #[serde(rename = "lastName")]
    pub last_name: String,
    pub points: f64,
    #[serde(rename = "eventPoints")]
    pub event_points: f64,
    #[serde(rename = "isWithdrawn")]
    pub is_withdrawn: bool,
    #[serde(rename = "isCut")]
    pub is_cut: bool,
    #[serde(rename = "isSuspended")]
    pub is_suspended: bool,
    #[serde(rename = "isFinal")]
    pub is_final: bool,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct MatchupShotcallerPick {
    #[serde(rename = "competitorId")]
    pub competitor_id: i64,
    #[serde(rename = "tournamentPositionId")]
    pub tournament_position_id: i64,
    #[serde(rename = "workoutId")]
    pub workout_id: i64,
    #[serde(rename = "firstName")]
    pub first_name: String,
    #[serde(rename = "lastName")]
    pub last_name: String,
    #[serde(rename = "eventPoints")]
    pub points: f64,
    #[serde(rename = "isWithdrawn")]
    pub is_withdrawn: bool,
    #[serde(rename = "isCut")]
    pub is_cut: bool,
    #[serde(rename = "isSuspended")]
    pub is_suspended: bool,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct MatchupDetail {
    #[serde(rename = "menPoints")]
    pub men_points: f64,
    #[serde(rename = "womenPoints")]
    pub women_points: f64,
    #[serde(rename = "menPlayers")]
    pub men_players: Vec<MatchupPick>,
    #[serde(rename = "womenPlayers")]
    pub women_players: Vec<MatchupPick>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct MatchupShotcallerDetail {
    pub points: f64,
    pub players: Vec<MatchupShotcallerPick>,
    #[serde(rename = "propPoints")]
    pub prop_points: f64,
    #[serde(rename = "propPicks")]
    pub prop_picks: Vec<PropMatchupDetail>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct WorkoutPredictionResponse {
    pub competitor: String,
    pub picks: i64,
    pub percentile: f64,
    #[serde(rename = "genderId")]
    pub gender_id: i64,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct WorkoutPredictionCountResponse {
    pub gender_id: i64,
    pub count: i64,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct LeaderboardMatchupResponse {
    #[serde(rename = "lockedEvents")]
    pub locked_events: u64,
    #[serde(rename = "userMatchup")]
    pub user_matchup: MatchupDetail,
    #[serde(rename = "competitorMatchup")]
    pub competitor_matchup: MatchupDetail,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct LeaderboardMatchupShotcallerResponse {
    #[serde(rename = "userMatchup")]
    pub user_matchup: MatchupShotcallerDetail,
    pub workouts: Vec<WorkoutResponse>,
    #[serde(rename = "competitorMatchup")]
    pub competitor_matchup: MatchupShotcallerDetail,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct LeaderboardMetadataData {
    pub competition_id: u64,
    pub competition_name: String,
    pub competition_logo: String,
    pub tournament_name: String,
    pub locked_events: u64,
    pub tournament_type_id: u64,
    pub pick_count: i64,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct LeaderboardScoreData {
    pub points: Option<f64>,
    pub ordinal: u64,
    pub rank: u64,
    pub competitor_id: u64,
    pub gender_id: u64,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CompetitionLeaderboardResponse {
    pub competitor_id: i64,
    pub competition_id: i64,
    pub gender_id: i64,
    pub first_name: String,
    pub last_name: String,
    pub points: f64,
    pub finishes: Vec<f64>,
    pub placement: i64,
    pub is_withdrawn: bool,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct LeaderboardTop10ScoreData {
    pub points: Option<f64>,
    pub rank: i64,
    pub competitor_id: u64,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct LeaderboardShotCallerScoreData {
    pub competitor_id: u64,
    pub men_competitors: Vec<LeaderboardScores>,
    pub women_competitors: Vec<LeaderboardScores>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct LeaderboardPicks {
    pub competitor_id: i64,
    pub rank: i64,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct LeaderboardShotcallerPicks {
    #[serde(rename = "competitorId")]
    pub competitor_id: i64,
    #[serde(rename = "tournamentPositionId")]
    pub tournament_position_id: i64,
    #[serde(rename = "workoutId")]
    pub workout_id: i64,
    #[serde(rename = "firstName")]
    pub first_name: String,
    #[serde(rename = "lastName")]
    pub last_name: String,
    pub points: f64,
    #[serde(rename = "isWithdrawn")]
    pub is_withdrawn: bool,
    #[serde(rename = "isCut")]
    pub is_cut: bool,
    #[serde(rename = "isSuspended")]
    pub is_suspended: bool,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct LeaderboardScores {
    pub points: f64,
    pub ordinal: i64,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct LeaderboardTournamentUserData {
    pub tournament_user_id: u64,
    pub display_name: String,
    pub avatar: String,
    pub men_competitor_ids: Vec<LeaderboardPicks>,
    pub women_competitor_ids: Vec<LeaderboardPicks>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct LeaderboardShotcallerTournamentUserData {
    pub tournament_user_id: i64,
    pub competitors: Vec<LeaderboardShotcallerPicks>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct LeaguePosition {
    #[serde(rename = "positionId")]
    pub position_id: i64,
    pub name: String,
    pub abbreviation: String,
    #[serde(rename = "imageUrl")]
    pub image_url: String,
    pub ordinal: i64,
    #[serde(rename = "allowedPositions")]
    pub allowed_positions: Option<Vec<i64>>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct UserLeaguesResponse {
    #[serde(rename = "tournamentUserId")]
    pub tournament_user_id: u64,
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,
    pub competition: String,
    #[serde(rename = "competitionId")]
    pub competition_id: u64,
    pub tournament: String,
    #[serde(rename = "tournamentId")]
    pub tournament_id: u64,
    #[serde(rename = "commissionerId")]
    pub commissioner_id: i64,
    pub logo: String,
    #[serde(rename = "lockedEvents")]
    pub locked_events: u64,
    #[serde(rename = "isActive")]
    pub is_active: bool,
    #[serde(rename = "isComplete")]
    pub is_complete: bool,
    #[serde(rename = "tournamentTypeId")]
    pub tournament_type_id: u64,
    #[serde(rename = "pickCount")]
    pub pick_count: Option<i64>,
    pub positions: Vec<LeaguePosition>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct UserLeaguesTopPicksDataResponse {
    #[serde(rename = "competitorId")]
    pub competitor_id: u64,
    #[serde(rename = "genderId")]
    pub gender_id: i64,
    pub rank: i64,
    pub id: i64,
    #[serde(rename = "tournamentPositionId")]
    pub tournament_position_id: u64,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct UserLeaguesPicksDataResponse {
    #[serde(rename = "competitorId")]
    pub competitor_id: u64,
    #[serde(rename = "workoutId")]
    pub workout_id: Option<i64>,
    pub id: i64,
    #[serde(rename = "tournamentPositionId")]
    pub tournament_position_id: u64,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct PickCompetitor {
    #[serde(rename = "competitorId")]
    pub competitor_id: i64,
    pub rank: i64,
    pub id: i64,
    #[serde(rename = "tournamentPositionId")]
    pub tournament_position_id: i64,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CompetitorPick {
    #[serde(rename = "competitorId")]
    pub competitor_id: u64,
    pub id: i64,
    pub rank: i64,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct PositionPicks {
    #[serde(rename = "competitorId")]
    pub competitor_id: u64,
    #[serde(rename = "positionId")]
    pub position_id: u64,
    pub id: i64,
    #[serde(rename = "workoutId")]
    pub workout_id: i64,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct UserLeaguesPicksResponse {
    #[serde(rename = "tournamentUserId")]
    pub tournament_user_id: u64,
    #[serde(rename = "menPicks")]
    pub men_picks: Vec<CompetitorPick>,
    #[serde(rename = "womenPicks")]
    pub women_picks: Vec<CompetitorPick>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct WorkoutStage {
    pub ordinal: i64,
    #[serde(rename = "timeCap")]
    pub time_cap: Option<String>,
    #[serde(rename = "stageType")]
    pub stage_type: String,
    pub movements: Option<Vec<WorkoutMovement>>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct WorkoutMovement {
    pub ordinal: i64,
    pub name: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct WorkoutResponse {
    pub id: i64,
    pub name: String,
    pub ordinal: i64,
    pub location: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "startTime")]
    pub start_time: String,
    #[serde(rename = "isActive")]
    pub is_active: bool,
    #[serde(rename = "isComplete")]
    pub is_complete: bool,
    #[serde(rename = "sponsorLogo")]
    pub sponsor_logo: Option<String>,
    #[serde(rename = "sponsorLogoDark")]
    pub sponsor_logo_dark: Option<String>,
    pub sponsor: Option<String>,
    #[serde(rename = "sponsorLink")]
    pub sponsor_link: Option<String>,
    pub stages: Option<Vec<WorkoutStage>>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct UserLeagueTournamentCompetitionStatus {
    pub is_active: bool,
    pub is_complete: bool,
    pub locked_events: i64,
    pub tournament_type_id: i64,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ShotCallerPicksResponse {
    pub athletes: Vec<LeagueAthletesResponse>,
    pub workouts: Vec<WorkoutResponse>,
    #[serde(rename = "menPicks")]
    pub men_picks: Vec<CompetitorPick>,
    #[serde(rename = "womenPicks")]
    pub women_picks: Vec<CompetitorPick>,
    pub props: Vec<PropBet>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ShotCallerPicksBetaResponse {
    pub athletes: Vec<LeagueAthletesResponse>,
    pub workouts: Vec<WorkoutResponse>,
    pub picks: Vec<PositionPicks>,
    pub props: Vec<PropBet>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct PickPercentage {
    pub percentage: f64,
    #[serde(rename = "workoutId")]
    pub workout_id: i64,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct LeagueAthletesResponse {
    #[serde(rename = "competitorId")]
    pub competitor_id: u64,
    #[serde(rename = "genderId")]
    pub gender_id: u64,
    #[serde(rename = "firstName")]
    pub first_name: String,
    #[serde(rename = "lastName")]
    pub last_name: String,
    pub adp: f64,
    #[serde(rename = "pickPercentage")]
    pub pick_percentage: Vec<PickPercentage>,
    #[serde(rename = "isLocked")]
    pub is_locked: bool,
    #[serde(rename = "isWithdrawn")]
    pub is_withdrawn: bool,
    #[serde(rename = "isCut")]
    pub is_cut: bool,
    #[serde(rename = "isSuspended")]
    pub is_suspended: bool,
    #[serde(rename = "positionId")]
    pub position_id: i64,
    pub position: Option<String>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct PropBetOption {
    pub id: i64,
    #[serde(rename = "propBetId")]
    pub prop_bet_id: i64,
    pub name: String,
    #[serde(rename = "imageUrl")]
    pub image_url: String,
    pub points: f64,
    pub percentage: f64,
    #[serde(rename = "isPicked")]
    pub is_picked: bool,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct PropBet {
    pub id: i64,
    pub name: String,
    #[serde(rename = "startTime")]
    pub start_time: String,
    pub ordinal: i64,
    #[serde(rename = "isActive")]
    pub is_active: bool,
    #[serde(rename = "isComplete")]
    pub is_complete: bool,
    pub description: Option<String>,
    pub options: Vec<PropBetOption>,
}
