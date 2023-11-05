use super::competitor::Competitor;
use serde::Serialize;

#[derive(Serialize)]
pub struct EliteCompetitor {
    pub id: u64,
    pub competitor_id: u64,
    pub ww_rank: u64,
    pub bfriendly_rank: u64,
    pub watkins_rak: u64,
    pub grade: f32,
    pub adp: f32,

    pub competitor: Competitor,
}