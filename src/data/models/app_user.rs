use serde::Serialize;

use super::tournament_users::TournamentUsers;

#[derive(Serialize)]
pub struct AppUser {
    pub id: u64,
    pub username: String,
    pub firebase_id: String,
    pub email: String,
    pub profile_url: String,

    pub leagues: Option<Vec<TournamentUsers>>,
}
