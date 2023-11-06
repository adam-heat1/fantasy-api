use crate::{
    data::models::app_user::AppUser, handlers::account::UpdateUsernameRequest,
    repositories::app_user::AppUserRepository,
};
use sqlx::Error;

pub struct AccountService;

impl AccountService {
    pub async fn get_email_by_username(username: String) -> Result<String, Error> {
        AppUserRepository::fetch_email_by_username(username).await
    }

    pub async fn update_username(user: &UpdateUsernameRequest) -> Result<String, Error> {
        AppUserRepository::update_username(user).await
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
