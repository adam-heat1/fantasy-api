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
