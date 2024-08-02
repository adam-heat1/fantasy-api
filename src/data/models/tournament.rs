use super::{competition::Competition, tournament_type::TournamentType};
use serde::Serialize;

#[derive(Serialize)]
pub struct Tournament {
    pub id: u64,
    pub competition_id: u64,
    pub name: String,
    pub logo: Option<String>,
    pub tournament_type_id: u64,
    pub is_private: bool,
    pub passcode: Option<String>,
    pub commissioner_id: u64,
    pub entries: Option<u64>,
    pub pick_count: Option<i64>,

    pub competition: Option<Competition>,
    pub tournament_type: Option<TournamentType>,
}
