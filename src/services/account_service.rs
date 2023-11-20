use crate::{
    data::models::app_user::AppUser,
    handlers::account::{
        CreateAccountDomainModel, CreateAccountViewModel, UpdateUsernameViewModel,
    },
    repositories::{app_user::AppUserRepository, league::LeagueRepository},
};
use sqlx::Error;

pub struct AccountService;

impl AccountService {
    pub async fn get_email_by_username(username: String) -> Result<String, Error> {
        AppUserRepository::fetch_email_by_username(username).await
    }

    pub async fn update_username(user: &UpdateUsernameViewModel) -> Result<String, Error> {
        AppUserRepository::update_username(user).await
    }

    pub async fn create_account(
        user: &CreateAccountViewModel,
    ) -> Result<CreateAccountDomainModel, Error> {
        let mut user = user.clone();
        user.profile_url =
            "https://heat1storage.blob.core.windows.net/user/athlete-avatar.jpg".to_string();

        let user_id = AppUserRepository::create_app_user(&&user).await?;

        LeagueRepository::create_app_user(13, user_id).await?;
        LeagueRepository::create_app_user(14, user_id).await?;

        let new_user = CreateAccountDomainModel {
            id: user_id as u64,
            username: user.username,
            email: user.email,
            profile_url: user.profile_url,
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
