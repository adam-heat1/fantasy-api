use serde_derive::Serialize;

#[derive(Serialize, Clone, Debug)]
pub struct CreateAccountResponse {
    pub id: u64,
    pub username: String,
    pub email: String,
    #[serde(rename = "profileUrl")]
    pub profile_url: String,
}

#[derive(Serialize, Clone, Debug)]
pub struct GetAccountResponse {
    pub id: u64,
    pub username: String,
    pub email: String,
    #[serde(rename = "profileUrl")]
    pub profile_url: String,
}
