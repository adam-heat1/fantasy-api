use serde_derive::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate, Clone, Debug)]
pub struct GetCompetitionAthleteRequest {
    #[serde(rename = "competitionId")]
    pub competition_id: i64,
    #[serde(rename = "competitorId")]
    pub competitor_id: i64,
}
