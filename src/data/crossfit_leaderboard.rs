use serde::Serialize;
use serde_derive::Deserialize;

#[derive(Serialize, Deserialize, Debug)]
pub struct CrossfitLeaderboard {
    pub competition: String,
    #[serde(rename = "leaderboardRows")]
    pub leaderboard_rows: String,
}
