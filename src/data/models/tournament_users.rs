use super::{tournament::Tournament, app_user::AppUser};
use serde::Serialize;

#[derive(Serialize)]
pub struct TournamentUsers {
    pub id: u64,
    pub tournament_id: u64,
    pub user_id: u64,
    pub display_name: String,

    pub tournament: Tournament,
    pub user: AppUser,
}