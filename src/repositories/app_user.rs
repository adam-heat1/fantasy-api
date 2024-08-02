use sqlx::{Error, Row};

use crate::handlers::account::response_models::GetAccountResponse;
use crate::{
    data::{data_client::DataClient, models::app_user::AppUser},
    handlers::account::request_models::UpdateUsername,
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
        .bind(username.to_lowercase().trim())
        .fetch_one(&pool)
        .await?;

        let email = res.get("email");

        return Ok(email);
    }

    pub async fn fetch_is_new_username_valid(username: String) -> Result<bool, Error> {
        let pool = DataClient::connect().await?;

        let res = sqlx::query(
            "
            SELECT
                username
            FROM
                app_user
            WHERE
                LOWER(username) = $1
            ",
        )
        .bind(username.to_lowercase().trim())
        .fetch_all(&pool)
        .await;

        if res.unwrap().len() > 0 {
            return Ok(false);
        }

        return Ok(true);
    }

    pub async fn update_username(user: &UpdateUsername) -> Result<String, Error> {
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
        .bind(user.username.clone().trim())
        .bind(user.user_id as i64)
        .execute(&pool)
        .await?;

        if res.rows_affected() == 0 {
            return Err(Error::RowNotFound);
        }

        return Ok("Updated username".to_string());
    }

    pub async fn create_app_user(user: AppUser) -> Result<i64, Error> {
        let pool = DataClient::connect().await?;

        let res = sqlx::query(
            "
            INSERT INTO
                app_user
            (username, firebase_id, email, profile_url)
            VALUES
                ($1, $2, $3, $4)
            RETURNING
                id
            ",
        )
        .bind(user.username.trim())
        .bind(user.firebase_id.clone().trim())
        .bind(user.email.clone().trim())
        .bind(user.profile_url.clone().trim())
        .fetch_one(&pool)
        .await?;

        let id = res.get("id");

        return Ok(id);
    }

    pub async fn fetch_user_by_firebase_id(
        firebase_id: String,
    ) -> Result<GetAccountResponse, Error> {
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
        .bind(firebase_id.trim())
        .fetch_one(&pool)
        .await?;

        let user = GetAccountResponse {
            id: res.get::<i64, _>("id") as u64,
            username: res.get("username"),
            email: res.get("email"),
            profile_url: res.get("profile_url"),
        };

        return Ok(user);
    }

    pub async fn fetch_user_by_user_id(user_id: u64) -> Result<GetAccountResponse, Error> {
        let pool = DataClient::connect().await?;

        let res = sqlx::query(
            "
            SELECT
                id,
                username,
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

        let user = GetAccountResponse {
            id: res.get::<i64, _>("id") as u64,
            username: res.get("username"),
            email: res.get("email"),
            profile_url: res.get("profile_url"),
        };

        return Ok(user);
    }

    pub async fn update_profile_url(user_id: i64, image_url: String) -> Result<(), Error> {
        let pool = DataClient::connect().await?;

        let _ = sqlx::query("UPDATE app_user SET profile_url = $2 WHERE id = $1")
            .bind(user_id)
            .bind(format!(
                "https://storage.googleapis.com/heat1-assets-pub/user/{image_url}",
            ))
            .execute(&pool)
            .await?;

        return Ok(());
    }
}
