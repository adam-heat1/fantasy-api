use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Workout {
    pub id: i64,
    pub name: String,
    pub ordinal: i64,
    pub start_time: String,
    pub description: Option<String>,
    pub location: Option<String>,
    pub is_active: bool,
    pub is_complete: bool,
}
