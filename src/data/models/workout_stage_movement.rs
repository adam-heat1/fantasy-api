use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkoutStageMovement {
    pub id: i64,
    pub workout_stage_id: i64,
    pub ordinal: i64,
    pub name: String,
}
