use serde_derive::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate, Clone, Debug)]
pub struct CreateTournament {
    #[validate(length(min = 3))]
    pub name: String,
    #[validate(range(min = 1))]
    #[serde(rename = "userId")]
    pub user_id: i32,
    #[validate(range(min = 1))]
    #[serde(rename = "competitionId")]
    pub competition_id: i32,
    #[validate(range(min = 1))]
    #[serde(rename = "tournamentTypeId")]
    pub tournament_type_id: i32,
    #[serde(rename = "isPrivate")]
    pub is_private: bool,
    pub passcode: Option<String>,
}