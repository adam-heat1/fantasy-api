use super::{competition::Competition, competitor::Competitor};
use serde::Serialize;

#[derive(Serialize)]
pub struct Score {
    pub id: u64,
    pub competition_id: u64,
    pub competitor_id: u64,
    pub ordinal: u64,
    pub rank: u64,
    pub is_scaled: bool,
    pub crossfit_id: String,
    pub breakdown: String,
    pub heat: String,
    pub judge: String,
    pub lane: String,
    pub mobile_score_display: String,
    pub score_display:String,
    pub time: String,
    pub is_valid: bool,
    pub video: String,
    pub year: u64,
    pub points: u64,
    pub inserted_at: String,

    pub competition: Competition,
    pub competitor: Competitor,
}