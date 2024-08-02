use crate::services::crossfit::CrossfitService;
use actix_web::{post, web::ServiceConfig, HttpResponse, Responder};

pub fn configure(config: &mut ServiceConfig) {
    config.service(save_open_scores);
}

#[post("/open")]
pub async fn save_open_scores() -> impl Responder {
    CrossfitService::save_open_scores(2024, 1)
        .await
        .map_or_else(
            |e| {
                let error_message = format!("Error inserting open scores: {:?}", e);
                HttpResponse::InternalServerError().body(error_message)
            },
            |_| HttpResponse::Ok().finish(),
        )
}
