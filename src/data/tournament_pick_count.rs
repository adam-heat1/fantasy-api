use serde::Serialize;
use serde_derive::Deserialize;

#[derive(Serialize, Deserialize, Debug)]
pub struct TournamentPickCount {
    pub id: i64,
    pub pick_count: i64,
}
