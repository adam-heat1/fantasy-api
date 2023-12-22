use serde_derive::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct Username {
    #[validate(length(min = 3))]
    pub username: String,
}

#[derive(Deserialize, Validate, Clone, Debug)]
pub struct UpdateUsername {
    #[serde(rename = "userId")]
    pub user_id: i32,
    pub username: String,
}

#[derive(Deserialize, Validate, Clone, Debug)]
pub struct CreateAccount {
    #[validate(length(min = 3))]
    pub username: String,
    #[validate(length(min = 28, max = 56))]
    #[serde(rename = "firebaseId")]
    pub firebase_id: String,
    #[validate(email)]
    pub email: String,
}

#[derive(Deserialize, Validate, Clone, Debug)]
pub struct GetFirebaseUserRequest {
    #[serde(rename = "firebaseId")]
    pub firebase_id: String,
}

#[derive(Deserialize, Validate, Clone, Debug)]
pub struct GetUserRequest {
    #[serde(rename = "userId")]
    pub user_id: u64,
}
