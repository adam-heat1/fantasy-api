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
    #[serde(rename = "competitionId")]
    pub competition_id: u64,
    #[serde(rename = "tournamentTypeId")]
    pub tournament_type_id: u64,
    #[serde(rename = "isPrivate")]
    pub is_private: bool,
    pub passcode: Option<String>,
    pub entries: u64,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct LeaderboardEntry {
    #[serde(rename = "tournamentUserId")]
    pub tournament_user_id: u64,
    #[serde(rename = "displayName")]
    pub display_name: String,
    pub avatar: String,
    #[serde(rename = "menPoints")]
    pub men_points: f64,
    #[serde(rename = "womenPoints")]
    pub women_points: f64,
    pub points: f64,
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
pub struct LeaderboardMetadataData {
    pub competition_id: u64,
    pub competition_name: String,
    pub competition_logo: String,
    pub tournament_name: String,
    pub locked_events: u64,
    pub tournament_type_id: u64,
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
    pub logo: String,
    #[serde(rename = "lockedEvents")]
    pub locked_events: u64,
    #[serde(rename = "isActive")]
    pub is_active: bool,
    #[serde(rename = "isComplete")]
    pub is_complete: bool,
    #[serde(rename = "tournamentTypeId")]
    pub tournament_type_id: u64,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct UserLeaguesPicksDataResponse {
    #[serde(rename = "competitorId")]
    pub competitor_id: u64,
    pub rank: u64,
    #[serde(rename = "genderId")]
    pub gender_id: u64,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CompetitorPick {
    #[serde(rename = "competitorId")]
    pub competitor_id: u64,
    pub rank: u64,
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
pub struct WorkoutResponse {
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
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ShotCallerPicksResponse {
    pub athletes: Vec<LeagueAthletesResponse>,
    pub workouts: Vec<WorkoutResponse>,
    #[serde(rename = "menPicks")]
    pub men_picks: Vec<CompetitorPick>,
    #[serde(rename = "womenPicks")]
    pub women_picks: Vec<CompetitorPick>,
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

    #[serde(rename = "wwRank")]
    pub ww_rank: Option<u64>,

    #[serde(rename = "adp")]
    pub adp: Option<f64>,

    #[serde(rename = "isLocked")]
    pub is_locked: bool,

    #[serde(rename = "isWithdrawn")]
    pub is_withdrawn: bool,

    #[serde(rename = "isCut")]
    pub is_cut: bool,

    #[serde(rename = "isSuspended")]
    pub is_suspended: bool,
}
