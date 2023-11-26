use serde_derive::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct CreateTournamentResponse {
    pub name: String,
    #[serde(rename = "userId")]
    pub user_id: i32,
    #[serde(rename = "competitionId")]
    pub competition_id: i32,
    #[serde(rename = "tournamentTypeId")]
    pub tournament_type_id: i32,
    #[serde(rename = "isPrivate")]
    pub is_private: bool,
    pub passcode: Option<String>,
}