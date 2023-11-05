use serde::Serialize;

#[derive(Serialize)]
pub struct CompetitorStatus {
    pub id: u64,
    pub name: String,
    pub abbreviation: String,
}