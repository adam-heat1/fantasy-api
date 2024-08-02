use crate::handlers::account::request_models::GetFirebaseUserRequest;
use crate::{
    data::constants::ntfy,
    handlers::account::request_models::{
        CreateAccount, GetUserRequest, UpdateProfilePictureRequest, UpdateUsername, Username,
    },
    services::account::AccountService,
    utils::notification::spawn_notification,
};
use actix_web::{
    get, post, put,
    web::{Json, Path, Query, ServiceConfig},
    HttpResponse, Responder,
};
use serde_json::json;
use validator::Validate;

pub fn configure(config: &mut ServiceConfig) {
    config
        .service(get_email_by_username)
        .service(get_user)
        .service(get_firebase_user)
        .service(validate_new_username)
        .service(update_username)
        .service(create_account)
        .service(update_profile_picure);
}

#[get("/firebase/{firebaseId}")]
pub(crate) async fn get_firebase_user(path: Path<GetFirebaseUserRequest>) -> impl Responder {
    let firebase_id = &path.firebase_id;
    AccountService::get_user_by_firebase_id(firebase_id)
        .await
        .map_or_else(
            |e| {
                if e.to_string().to_lowercase().contains("no rows returned") {
                    return HttpResponse::NotFound().body("No user found with that firebase id");
                }
                let error_message = format!(
                    "Error fetching user by firebase id: {}: -> {:?}",
                    firebase_id, e
                );
                spawn_notification(ntfy::ERROR.to_string(), error_message);

                HttpResponse::InternalServerError().body("Error fetching user by firebase id")
            },
            |user| HttpResponse::Ok().json(json!(user)),
        )
}

#[get("/user/{userId}")]
pub(crate) async fn get_user(path: Path<GetUserRequest>) -> impl Responder {
    let user_id = &path.user_id;
    AccountService::get_user_by_user_id(user_id)
        .await
        .map_or_else(
            |e| {
                if e.to_string().to_lowercase().contains("no rows returned") {
                    return HttpResponse::NotFound().body("No user found with that user id");
                }
                let error_message = format!(
                    "Error fetching user by firebase id: {}: -> {:?}",
                    user_id, e
                );
                spawn_notification(ntfy::ERROR.to_string(), error_message);

                HttpResponse::InternalServerError().body("Error fetching user by user id")
            },
            |user| HttpResponse::Ok().json(json!(user)),
        )
}

#[get("/email")]
pub(crate) async fn get_email_by_username(req: Query<Username>) -> impl Responder {
    if req.0.validate().is_err() {
        let message = format!(
            "get_email_by_username: -> {:?}",
            req.validate().unwrap_err()
        );
        spawn_notification(ntfy::ERROR.to_string(), message);

        return HttpResponse::BadRequest().body("Invalid get email by username request");
    }

    let username = &req.username;

    AccountService::get_email_by_username(username.to_string())
        .await
        .map_or_else(
            |e| {
                if e.to_string().to_lowercase().contains("no rows returned") {
                    return HttpResponse::NotFound().body("No user found with that username");
                }
                let error_message =
                    format!("Error fetching email by username: {}: -> {:?}", username, e);
                spawn_notification(ntfy::ERROR.to_string(), error_message);

                HttpResponse::InternalServerError().body("Error fetching email by username")
            },
            |email| HttpResponse::Ok().body(email),
        )
}

#[get("/username/validate")]
pub(crate) async fn validate_new_username(req: Query<Username>) -> impl Responder {
    if req.0.validate().is_err() {
        let message = format!(
            "validate_new_username: -> {:?}",
            req.validate().unwrap_err()
        );
        spawn_notification(ntfy::ERROR.to_string(), message);

        return HttpResponse::BadRequest().body("Invalid validate username request");
    }

    let username = &req.username;

    AccountService::validate_new_username(username.to_string())
        .await
        .map_or_else(
            |e| {
                let error_message = format!("Error validating username: {}: -> {:?}", username, e);
                spawn_notification(ntfy::ERROR.to_string(), error_message);

                HttpResponse::InternalServerError().body("Error validating username")
            },
            |is_valid| HttpResponse::Ok().json(is_valid),
        )
}

#[put("/username")]
pub(crate) async fn update_username(body: Json<UpdateUsername>) -> impl Responder {
    if body.validate().is_err() {
        let message = format!("update_username: -> {:?}", body.validate().unwrap_err());
        spawn_notification(ntfy::ERROR.to_string(), message);

        return HttpResponse::BadRequest().body("Invalid update username request");
    }

    let user: &UpdateUsername = &body.into_inner();

    AccountService::update_username(user).await.map_or_else(
        |e| {
            if e.to_string().to_lowercase().contains("no rows returned") {
                return HttpResponse::NotFound().body("No user found with that username");
            }

            spawn_notification(
                ntfy::ERROR.to_string(),
                format!("update_username: {:?}: -> {:?}", user, e),
            );

            HttpResponse::InternalServerError().body("Error updating username")
        },
        |message| HttpResponse::Ok().body(message),
    )
}

#[post("/")]
pub(crate) async fn create_account(body: Json<CreateAccount>) -> impl Responder {
    if body.validate().is_err() {
        let message = format!("create_account: -> {:?}", body.validate().unwrap_err());
        spawn_notification(ntfy::ERROR.to_string(), message);

        return HttpResponse::BadRequest().body("Invalid create account request");
    }

    let user: &CreateAccount = &body.into_inner();

    AccountService::create_account(user).await.map_or_else(
        |e| {
            spawn_notification(
                ntfy::ERROR.to_string(),
                format!("Error creating account: {:?}: -> {:?}", user, e),
            );

            HttpResponse::InternalServerError().body("Error updating username")
        },
        |account| HttpResponse::Ok().json(json!(account)),
    )
}

#[post("/profile/{userId}/{imageUrl}")]
pub(crate) async fn update_profile_picure(
    req: Path<UpdateProfilePictureRequest>,
) -> impl Responder {
    if req.validate().is_err() {
        let message = format!(
            "update_profile_picure: -> {:?}",
            req.validate().unwrap_err()
        );
        spawn_notification(ntfy::ERROR.to_string(), message);

        return HttpResponse::BadRequest().body("Invalid create account request");
    }

    AccountService::update_profile_picture(req.user_id, req.image_url.clone())
        .await
        .map_or_else(
            |e| {
                spawn_notification(
                    ntfy::ERROR.to_string(),
                    format!("Error creating account: {:?}: -> {:?}", req, e),
                );

                HttpResponse::InternalServerError().body("Error updating username")
            },
            |_| {
                HttpResponse::Ok().body(format!(
                    "https://storage.googleapis.com/heat1-assets-pub/user/{}",
                    req.image_url
                ))
            },
        )
}
