use crate::{
    data::models::app_user::AppUser,
    handlers::account::{
        request_models::{CreateAccount, UpdateUsername},
        response_models::CreateAccountResponse,
    },
    repositories::{app_user::AppUserRepository, league::LeagueRepository},
};
use sqlx::Error;

pub struct AccountService;

impl AccountService {
    pub async fn get_email_by_username(username: String) -> Result<String, Error> {
        AppUserRepository::fetch_email_by_username(username).await
    }

    pub async fn validate_new_username(username: String) -> Result<bool, Error> {
        AppUserRepository::fetch_is_new_username_valid(username).await
    }

    pub async fn update_username(user: &UpdateUsername) -> Result<String, String> {
        let is_username_valid =
            AppUserRepository::fetch_is_new_username_valid(user.username.clone())
                .await
                .unwrap();
        if !is_username_valid {
            return Err("Username is already taken. Please choose another.".to_string());
        }
        AppUserRepository::update_username(user)
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn create_account(user: &CreateAccount) -> Result<CreateAccountResponse, Error> {
        let user = user.clone();
        let profile_url = "https://heat1storage.blob.core.windows.net/user/athlete-avatar.jpg";

        let new_user = AppUser {
            id: 0,
            username: user.username.clone(),
            firebase_id: user.firebase_id.clone(),
            email: user.email.clone(),
            profile_url: profile_url.to_string(),
            leagues: None,
        };

        let user_id = AppUserRepository::create_app_user(new_user).await?;

        LeagueRepository::create_app_user(13, user_id).await?;
        LeagueRepository::create_app_user(14, user_id).await?;

        let new_user = CreateAccountResponse {
            id: user_id as u64,
            username: user.username,
            email: user.email,
            profile_url: profile_url.to_string(),
        };

        Ok(new_user)
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
