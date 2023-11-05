use serde::Serialize;

#[derive(Serialize)]
pub struct AppUser {
    pub id: u64,
    pub username: String,
    pub firebase_id: String,
    pub email: String,
    pub profile_url: String,
}