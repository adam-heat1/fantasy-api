use super::{competition::Competition, tournament_type::TournamentType};
use serde::Serialize;

#[derive(Serialize)]
pub struct Tournament {
    pub id: u64,
    pub competition_id: u64,
    pub name: String,
    pub logo: String,
    pub tournament_type_id: u64,
    pub locked_events: u64,

    pub competition: Option<Competition>,
    pub tournament_type: Option<TournamentType>,
}