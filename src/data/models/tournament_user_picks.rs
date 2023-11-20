use super::{app_user::AppUser, competitor::Competitor};
use serde::Serialize;

#[derive(Serialize)]
pub struct TournamentUserPicks {
    pub id: u64,
    pub tournament_user_id: u64,
    pub competitor_id: u64,
    pub ordinal: u64,
    pub last_updated: String,

    pub tournament_user: AppUser,
    pub competitor: Competitor,
}