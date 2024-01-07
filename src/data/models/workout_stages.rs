use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkoutStages {
    pub id: i64,
    pub workout_id: i64,
    pub ordinal: i64,
    pub time_cap: Option<String>,
    pub stage_type: String,
}
