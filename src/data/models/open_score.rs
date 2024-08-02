use serde::Serialize;

#[derive(Serialize)]
pub struct OpenScore {
    pub id: i64,
    pub labels: Vec<String>,
    pub men_data: Vec<i64>,
    pub women_data: Vec<i64>,
    pub label: String,
    pub last_updated: String,
}
