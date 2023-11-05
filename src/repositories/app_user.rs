use sqlx::{Error, Row};

use crate::data::{data_client::DataClient, models::app_user::AppUser};

pub struct AppUserRepository;

impl AppUserRepository {
    pub async fn fetch_email_by_username(username: String) -> Result<String, Error> {
        let pool = DataClient::connect().await?;

        let res = sqlx::query(
            "
            SELECT 
                email
            FROM 
                app_user
            WHERE
                LOWER(username) = $1
            ",
        )
        .bind(username.to_lowercase())
        .fetch_one(&pool)
        .await?;

        let email = res.get("email");

        return Ok(email);
    }

    pub async fn fetch_user_by_firebase_id(firebase_id: String) -> Result<AppUser, Error> {
        let pool = DataClient::connect().await?;

        let res = sqlx::query(
            "
            SELECT 
                id,
                username,
                firebase_id,
                email,
                profile_url
            FROM 
                app_user
            WHERE
                firebase_id = $1
            ",
        )
        .bind(firebase_id)
        .fetch_one(&pool)
        .await?;

        let user = AppUser{
            id: res.get::<i64, _>("id") as u64,
            username: res.get("username"),
            firebase_id: res.get("firebase_id"),
            email: res.get("email"),
            profile_url: res.get("profile_url"),
        };

        return Ok(user);
    }

    pub async fn fetch_user_by_user_id(user_id: u64) -> Result<AppUser, Error> {
        let pool = DataClient::connect().await?;

        let res = sqlx::query(
            "
            SELECT 
                id,
                username,
                firebase_id,
                email,
                profile_url
            FROM 
                app_user
            WHERE
                id = $1
            ",
        )
        .bind(user_id as i64)
        .fetch_one(&pool)
        .await?;

        let user = AppUser{
            id: res.get::<i64, _>("id") as u64,
            username: res.get("username"),
            firebase_id: res.get("firebase_id"),
            email: res.get("email"),
            profile_url: res.get("profile_url"),
        };

        return Ok(user);
    }
}
