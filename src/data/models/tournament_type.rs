use serde::Serialize;

#[derive(Serialize)]
pub struct TournamentType {
    pub id: u64,
    pub name: String,
}