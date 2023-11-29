use crate::{
    data::constants::ntfy, handlers::league::request_models::CreateLeague,
    services::league::LeagueService, utils::notification::spawn_notification,
};
use actix_web::{
    post,
    web::{Json, ServiceConfig},
    HttpResponse, Responder,
};
use validator::Validate;

pub fn configure(config: &mut ServiceConfig) {
    config.service(create_league);
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
