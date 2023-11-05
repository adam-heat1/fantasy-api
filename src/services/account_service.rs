use crate::{repositories::app_user::AppUserRepository, data::models::app_user::AppUser};
use sqlx::Error;

pub struct AccountService;

impl AccountService {
    pub async fn get_email_by_username(username: String) -> Result<String, Error> {
        let email = AppUserRepository::fetch_email_by_username(username).await?;

        Ok(email)
    }

    pub async fn get_user_by_firebase_id(firebase_id: String) -> Result<AppUser, Error> {
        let user = AppUserRepository::fetch_user_by_firebase_id(firebase_id).await?;

        Ok(user)
    }

    pub async fn get_user_by_user_id(user_id: u64) -> Result<AppUser, Error> {
        let user = AppUserRepository::fetch_user_by_user_id(user_id).await?;

        Ok(user)
    }
}