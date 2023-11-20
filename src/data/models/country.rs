use serde::Serialize;

#[derive(Serialize)]
pub struct Country {
    pub id: u64,
    pub name: String,
    pub code: String,
}