use crate::{
    handlers::competition::request_models::{CreateCompetitionCompetitor, GetCompetitor},
    services::competition::CompetitionService,
};
use actix_web::{
    get, post,
    web::{Json, Path, ServiceConfig},
    HttpResponse, Responder,
};
use validator::Validate;

pub fn configure(config: &mut ServiceConfig) {
    config
        .service(get_competitors)
        .service(get_active_competitions)
        .service(get_active_beta_competitions)
        .service(create_competittion_competitor);
}

#[get("/active/beta")]
pub(crate) async fn get_active_beta_competitions() -> impl Responder {
    CompetitionService::fetch_active_beta_competitions()
        .await
        .map_or_else(
            |e| HttpResponse::InternalServerError().body(e.to_string()),
            |competitions| HttpResponse::Ok().json(competitions),
        )
}

#[get("/active")]
pub(crate) async fn get_active_competitions() -> impl Responder {
    CompetitionService::fetch_active_competitions()
        .await
        .map_or_else(
            |e| HttpResponse::InternalServerError().body(e.to_string()),
            |competitions| HttpResponse::Ok().json(competitions),
        )
}

#[get("/competitor/{name}")]
pub(crate) async fn get_competitors(req: Path<GetCompetitor>) -> impl Responder {
    if req.validate().is_err() {
        return HttpResponse::BadRequest().body("Invalid competitor request");
    }

    let name = req.clone().name;

    CompetitionService::fetch_new_competitor(name)
        .await
        .map_or_else(
            |e| HttpResponse::InternalServerError().body(e.to_string()),
            |competitors| HttpResponse::Ok().json(competitors),
        )
}

#[post("/competitor")]
pub async fn create_competittion_competitor(
    body: Json<CreateCompetitionCompetitor>,
) -> impl Responder {
    if body.validate().is_err() {
        let message = format!(
            "create_competition_competitor: -> {:?}",
            body.validate().unwrap_err()
        );

        return HttpResponse::BadRequest().body(message);
    }

    let cc: CreateCompetitionCompetitor = body.into_inner();

    CompetitionService::insert_competition_competitor(cc.competition_id, cc.competitor_id)
        .await
        .map_or_else(
            |e| {
                let error_message = format!("Error creating competition competitor: {:?}", e);

                HttpResponse::InternalServerError().body(error_message)
            },
            |_| HttpResponse::Ok().finish(),
        )
}
