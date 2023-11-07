use crate::services::account_service::AccountService;
use actix_web::{
    get, put,
    web::{Json, Query, ServiceConfig},
    HttpResponse, Responder,
};
use serde_derive::Deserialize;

pub fn configure(config: &mut ServiceConfig) {
    config
        .service(get_email_by_username)
        .service(update_username);
}

#[derive(Deserialize)]
pub struct EmailRequest {
    username: String,
}

#[get("/email")]
pub(crate) async fn get_email_by_username(req: Query<EmailRequest>) -> impl Responder {
    let username = &req.username;
    if username.is_empty() {
        return HttpResponse::BadRequest().body("No username provided!");
    }

    AccountService::get_email_by_username(username.to_string())
        .await
        .map_or_else(
            |e| {
                log::error!("Error fetching email by username: {}: -> {:?}", username, e);

                if e.to_string().to_lowercase().contains("no rows returned") {
                    return HttpResponse::NotFound().body("No user found with that username!");
                }
                HttpResponse::InternalServerError().body("Error fetching email by username!")
            },
            |email| HttpResponse::Ok().body(email),
        )
}

#[derive(Deserialize, Clone)]
pub struct UpdateUsernameRequest {
    pub user_id: i32,
    pub username: String,
}

#[put("/username")]
pub(crate) async fn update_username(body: Json<UpdateUsernameRequest>) -> impl Responder {
    if body.username.is_empty() {
        return HttpResponse::BadRequest().body("No username provided!");
    }

    if body.user_id == 0 {
        return HttpResponse::BadRequest().body("No user id provided!");
    }

    let user: &UpdateUsernameRequest = &body.into_inner();

    //Handle success and failure paths
    AccountService::update_username(user).await.map_or_else(
        |e| {
            log::error!(
                "Error updating username for user: {}: -> {:?}",
                user.user_id,
                e
            );

            if e.to_string().to_lowercase().contains("no rows returned") {
                return HttpResponse::NotFound().body("No user found with that username!");
            }
            HttpResponse::InternalServerError().body("Error updating username!")
        },
        |email| HttpResponse::Ok().body(email),
    )
}

// #[post("/echo")]
// pub(crate) async fn echo(req_body: String) -> impl Responder {
//     HttpResponse::Ok().body(req_body)
// }

// pub(crate) async fn manual_hello() -> impl Responder {
//     // Test env "TARGET" which defined when `docker run`, or `gcloud run deploy --set-env-vars`
//     // Depend on your platform target. (See README.md)
//     let test_target = match env::var("TARGET") {
//         Ok(target) => format!("Hey {target}!"),
//         Err(_e) => "No TARGET env defined!".to_owned(),
//     };

//     // Response with test_target
//     HttpResponse::Ok().body(test_target)
// }
