use crate::{
    data::constants::ntfy, handlers::league::request_models::CreateTournament,
    utils::notification::spawn_notification,
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
pub(crate) async fn create_league(body: Json<CreateTournament>) -> impl Responder {
    if body.validate().is_err() {
        let message = format!("create_league: -> {:?}", body.validate().unwrap_err());
        spawn_notification(ntfy::ERROR.to_string(), message.clone());

        return HttpResponse::BadRequest().body("Invalid create league request");
    }

    // NewsService::get_news().await.map_or_else(
    //     |e| {
    //         let error_message = format!("Error getting news: -> {:?}", e);
    //         spawn_notification(ntfy::ERROR.to_string(), error_message.clone());
    //
    //         HttpResponse::InternalServerError().body("Error getting news")
    //     },
    //     |response| HttpResponse::Ok().json(response),
    // )
    return HttpResponse::Ok().finish();
}
