use super::{competition::Competition, competitor::Competitor};
use serde::Serialize;

#[derive(Serialize)]
pub struct CompetitionCompetitor {
    pub id: u64,
    pub competition_id: u64,
    pub competitor_id: u64,
    pub is_withdrawn: bool,
    pub is_cut: bool,
    pub is_suspended: bool,

    pub competition: Competition,
    pub competitor: Competitor,
}
