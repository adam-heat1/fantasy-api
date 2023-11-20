use serde::Serialize;

#[derive(Serialize)]
pub struct Region {
    pub id: u64,
    pub name: String,
    pub abbreviation: String,
    pub crossfit_id: u64,
    pub ordinal: u64,
}