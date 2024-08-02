use serde_derive::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate, Clone, Debug)]
pub struct GetPropsRequest {
    #[serde(rename = "competitionId")]
    pub competition_id: i64,
    #[serde(rename = "tournamentUserId")]
    pub tournament_user_id: i64,
}

#[derive(Deserialize, Validate, Clone, Debug)]
pub struct GetUserPropEntriesRequest {
    #[serde(rename = "userId")]
    pub user_id: i64,
}

#[derive(Deserialize, Validate, Clone, Debug)]
pub struct CreatePropPickRequest {
    #[serde(rename = "tournamentUserId")]
    pub tournament_user_id: i64,
    #[serde(rename = "propId")]
    pub prop_id: i64,
    #[serde(rename = "propOptionId")]
    pub prop_option_id: i64,
}

#[derive(Deserialize, Validate, Clone, Debug)]
pub struct PropMatchupRequest {
    #[validate(range(min = 1))]
    #[serde(rename = "userId")]
    pub user_id: i64,
    #[validate(range(min = 0))]
    #[serde(rename = "competitorId")]
    pub competitor_id: i64,
}

#[derive(Deserialize, Validate, Clone, Debug)]
pub struct PropStatusRequest {
    #[validate(range(min = 1))]
    #[serde(rename = "propBetId")]
    pub prop_bet_id: i64,
}
