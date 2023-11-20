use sqlx::{Error, Row};

use crate::{
    data::{data_client::DataClient, models::app_user::AppUser},
    handlers::account::{CreateAccountViewModel, UpdateUsernameViewModel},
};

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

    pub async fn update_username(user: &UpdateUsernameViewModel) -> Result<String, Error> {
        let pool = DataClient::connect().await?;

        let res = sqlx::query(
            "
            UPDATE 
                app_user
            SET 
                username = $1
            WHERE
                id = $2
            ",
        )
        .bind(user.username.clone())
        .bind(user.user_id as i64)
        .execute(&pool)
        .await?;

        if res.rows_affected() == 0 {
            return Err(Error::RowNotFound);
        }

        return Ok("Updated username".to_string());
    }

    pub async fn create_app_user(user: &CreateAccountViewModel) -> Result<i64, Error> {
        let pool = DataClient::connect().await?;

        let res = sqlx::query(
            "
            INSERT INTO
                app_user
            (username, firebase_id, email) 
            VALUES 
                ($1, $2, $3)
            RETURNING
                id
            ",
        )
        .bind(user.username.clone())
        .bind(user.firebase_id.clone())
        .bind(user.email.clone())
        .fetch_one(&pool)
        .await?;

        let id = res.get("id");

        return Ok(id);
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

        let user = AppUser {
            id: res.get::<i64, _>("id") as u64,
            username: res.get("username"),
            firebase_id: res.get("firebase_id"),
            email: res.get("email"),
            profile_url: res.get("profile_url"),
            leagues: vec![],
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

        let user = AppUser {
            id: res.get::<i64, _>("id") as u64,
            username: res.get("username"),
            firebase_id: res.get("firebase_id"),
            email: res.get("email"),
            profile_url: res.get("profile_url"),
            leagues: vec![],
        };

        return Ok(user);
    }
}
