use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Clone, Debug)]
pub struct PropBetOptions {
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

#[derive(Serialize, Clone, Debug)]
pub struct PropBetsResponse {
    pub id: i64,
    pub name: String,
    #[serde(rename = "startTime")]
    pub start_time: String,
    pub ordinal: i64,
    #[serde(rename = "isActive")]
    pub is_active: bool,
    #[serde(rename = "isComplete")]
    pub is_complete: bool,
    #[serde(rename = "workoutId")]
    pub workout_id: i64,
    #[serde(rename = "workoutName")]
    pub workout_name: String,
    #[serde(rename = "workoutOrdinal")]
    pub workout_ordinal: i64,
    pub options: Vec<PropBetOptions>,
}

#[derive(Serialize, Clone, Debug)]
pub struct PropPickResponse {
    pub id: i64,
    pub prop_option_id: i64,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct SelfVsWorldEventScore {
    pub rank: i64,
    pub points: f64,
    pub ordinal: i64,
    pub label: String,
    pub description: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct SelfVsWorldLeadeerboardEntry {
    pub index: i64,
    #[serde(rename = "displayName")]
    pub display_name: String,
    pub avatar: String,
    pub events: Vec<SelfVsWorldEventScore>,
    pub points: f64,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct PropLeaderboardEntry {
    #[serde(rename = "tournamentUserId")]
    pub tournament_user_id: i64,
    #[serde(rename = "displayName")]
    pub display_name: String,
    pub avatar: String,
    pub points: f64,
    #[serde(rename = "eventWins")]
    pub event_wins: i64,
}

#[derive(Serialize, Clone, Debug)]
pub struct PropLeaderboardResponse {
    pub tournament: String,
    pub competition: String,
    pub logo: String,
    pub leaderboard: Vec<PropLeaderboardEntry>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct PropUserMatchup {
    pub display_name: String,
    pub avatar: String,
    pub points: f64,
    #[serde(rename = "eventWins")]
    pub event_wins: i64,
    pub picks: Vec<PropMatchupDetail>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct PropMatchupDetail {
    pub ordinal: i64,
    pub points: f64,
    pub description: String,
    pub metadata: String,
    pub name: String,
    pub workout: String,
    #[serde(rename = "imageUrl")]
    pub image_url: String,
    #[serde(rename = "isLocked")]
    pub is_locked: bool,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct PropMatchupResponse {
    #[serde(rename = "userMatchup")]
    pub user_matchup: PropUserMatchup,
    #[serde(rename = "competitorMatchup")]
    pub competitor_matchup: PropUserMatchup,
}
