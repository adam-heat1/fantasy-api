use serde::Serialize;

#[derive(Serialize)]
pub struct Gender {
    pub id: u64,
    pub name: String,
    pub abbreviation: String,
}