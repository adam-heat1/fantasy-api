use serde_derive::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate, Clone, Debug)]
pub struct GetCompetitor {
    #[validate(length(min = 1))]
    pub name: String,
}

#[derive(Deserialize, Validate, Clone, Debug)]
pub struct CreateCompetitionCompetitor {
    #[validate(range(min = 28))]
    #[serde(rename = "competitionId")]
    pub competition_id: i64,
    #[validate(range(min = 1))]
    #[serde(rename = "competitorId")]
    pub competitor_id: i64,
}
