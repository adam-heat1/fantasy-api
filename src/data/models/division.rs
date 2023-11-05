use serde::Serialize;

#[derive(Serialize)]
pub struct Division {
    pub id: u64,
    pub name: String,
    pub crossfit_id: u64,
    pub ordinal: u64,
}