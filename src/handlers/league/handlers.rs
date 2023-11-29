use crate::{
    data::constants::ntfy,
    handlers::league::request_models::{CreateLeague, OpenLeague},
    services::league::LeagueService,
    utils::notification::spawn_notification,
};
use actix_web::web::Query;
use actix_web::{
    get, post,
    web::{Json, ServiceConfig},
    HttpResponse, Responder,
};
use validator::Validate;

pub fn configure(config: &mut ServiceConfig) {
    config.service(get_open_leagues).service(create_league);
}

#[get("/open")]
pub(crate) async fn get_open_leagues(req: Query<OpenLeague>) -> impl Responder {
    if req.0.validate().is_err() {
        let message = format!("get_open_leagues: -> {:?}", req.validate().unwrap_err());
        spawn_notification(ntfy::ERROR.to_string(), message);

        return HttpResponse::BadRequest().body("Invalid open leagues request");
    }

    let user_id = &req.user_id;
    let competition_id = &req.competition_id;

    LeagueService::get_open_leagues(competition_id, user_id)
        .await
        .map_or_else(
            |e| {
                let error_message = format!(
                    "Error getting open leagues: {} - {}: -> {:?}",
                    competition_id, user_id, e
                );
                spawn_notification(ntfy::ERROR.to_string(), error_message);

                HttpResponse::InternalServerError().body("Error getting open leagues")
            },
            |leagues| HttpResponse::Ok().json(leagues),
        )
}

#[post("")]
pub(crate) async fn create_league(body: Json<CreateLeague>) -> impl Responder {
    if body.validate().is_err() {
        let message = format!("create_league: -> {:?}", body.validate().unwrap_err());
        spawn_notification(ntfy::ERROR.to_string(), message);

        return HttpResponse::BadRequest().body("Invalid create league request");
    }

    LeagueService::create_league(&body.0).await.map_or_else(
        |e| {
            let message = format!("create_league: -> {:?}", e);
            spawn_notification(ntfy::ERROR.to_string(), message);

            HttpResponse::InternalServerError().body("Error creating league")
        },
        |response| HttpResponse::Ok().json(response),
    )
}
