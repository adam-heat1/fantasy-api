use crate::{
    data::constants::ntfy, handlers::athlete::request_models::GetCompetitionAthleteRequest,
    services::athlete::AthleteService, utils::notification::spawn_notification,
};
use actix_web::{
    get,
    web::{Path, ServiceConfig},
    HttpResponse, Responder,
};
use serde_json::json;

pub fn configure(config: &mut ServiceConfig) {
    config.service(get_competition_athlete);
}

#[get("/{competitionId}/{competitorId}")]
pub(crate) async fn get_competition_athlete(
    path: Path<GetCompetitionAthleteRequest>,
) -> impl Responder {
    AthleteService::get_competition_competitor(path.competition_id, path.competitor_id)
        .await
        .map_or_else(
            |e| {
                if e.to_string().to_lowercase().contains("no rows returned") {
                    return HttpResponse::NotFound().body("No athlete found");
                }
                let error_message = format!(
                    "Error fetching athlete: {} - {}: -> {:?}",
                    &path.competition_id, &path.competitor_id, e
                );
                spawn_notification(ntfy::ERROR.to_string(), error_message);

                HttpResponse::InternalServerError().body("Error fetching athlete")
            },
            |user| HttpResponse::Ok().json(json!(user)),
        )
}
