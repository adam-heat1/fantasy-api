use std::env;

use crate::services::account_service::AccountService;
use actix_web::{
    get, post, put,
    web::{Json, Query, ServiceConfig},
    HttpResponse, Responder,
};
use serde_derive::{Deserialize, Serialize};
use serde_json::json;

pub fn configure(config: &mut ServiceConfig) {
    config
        .service(get_email_by_username)
        .service(update_username)
        .service(create_account);
}

#[derive(Deserialize)]
pub struct EmailViewModel {
    username: String,
}

#[get("/email")]
pub(crate) async fn get_email_by_username(req: Query<EmailViewModel>) -> impl Responder {
    let username = &req.username;
    let unknown_error_provider =
        env::var("NTFY_UNKNOWN_ERROR").expect("NTFY_UNKNOWN_ERROR must be set");
    let client = reqwest::blocking::Client::new();
    let _ = client
        .post(format!("ntfy.sh/{}", unknown_error_provider))
        .body(username.to_string())
        .send();
    if username.is_empty() {
        return HttpResponse::BadRequest().body("No username provided!");
    }

    AccountService::get_email_by_username(username.to_string())
        .await
        .map_or_else(
            |e| {
                let error_message =
                    format!("Error fetching email by username: {}: -> {:?}", username, e);
                log::error!("Error fetching email by username: {}: -> {:?}", username, e);

                if e.to_string().to_lowercase().contains("no rows returned") {
                    return HttpResponse::NotFound().body("No user found with that username!");
                }
                let unknown_error_provider =
                    env::var("NTFY_UNKNOWN_ERROR").expect("NTFY_UNKNOWN_ERROR must be set");
                let client = reqwest::blocking::Client::new();
                let _ = client
                    .post(format!("ntfy.sh/{}", unknown_error_provider))
                    .body(error_message)
                    .send();
                HttpResponse::InternalServerError().body("Error fetching email by username!")
            },
            |email| HttpResponse::Ok().body(email),
        )
}

#[derive(Deserialize, Clone, Debug)]
pub struct UpdateUsernameViewModel {
    pub user_id: i32,
    pub username: String,
}

#[put("/username")]
pub(crate) async fn update_username(body: Json<UpdateUsernameViewModel>) -> impl Responder {
    if body.username.is_empty() {
        return HttpResponse::BadRequest().body("No username provided!");
    }

    if body.user_id == 0 {
        return HttpResponse::BadRequest().body("No user id provided!");
    }

    let user: &UpdateUsernameViewModel = &body.into_inner();

    AccountService::update_username(user).await.map_or_else(
        |e| {
            log::error!("Error updating username for user: {:?}: -> {:?}", user, e);

            if e.to_string().to_lowercase().contains("no rows returned") {
                return HttpResponse::NotFound().body("No user found with that username!");
            }
            HttpResponse::InternalServerError().body("Error updating username!")
        },
        |message| HttpResponse::Ok().body(message),
    )
}

#[derive(Deserialize, Clone, Debug)]
pub struct CreateAccountViewModel {
    pub username: String,
    pub firebase_id: String,
    pub email: String,
    pub profile_url: String,
}

#[derive(Serialize, Clone, Debug)]
pub struct CreateAccountDomainModel {
    pub id: u64,
    pub username: String,
    pub email: String,
    pub profile_url: String,
}

#[post("/")]
pub(crate) async fn create_account(body: Json<CreateAccountViewModel>) -> impl Responder {
    if body.username.trim().is_empty() {
        return HttpResponse::BadRequest().body("No username provided!");
    }
    if body.firebase_id.trim().is_empty() {
        return HttpResponse::BadRequest().body("No firebase id provided!");
    }
    if body.email.trim().is_empty() {
        return HttpResponse::BadRequest().body("No email provided!");
    }

    let user: &CreateAccountViewModel = &body.into_inner();

    AccountService::create_account(user).await.map_or_else(
        |e| {
            log::error!("Error creating account: {:?}: -> {:?}", user, e);
            HttpResponse::InternalServerError().body("Error updating username!")
        },
        |account| HttpResponse::Ok().json(json!(account)),
    )
}
