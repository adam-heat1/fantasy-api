use serde::Serialize;
use serde_json::Value;

use super::tournament::Tournament;

#[derive(Serialize)]
pub struct TournamentWorkouts {
    pub id: u64,
    pub tournament_id: u64,
    pub workouts: Value,

    pub tournament: Tournament,
}
