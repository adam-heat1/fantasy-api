use crate::services::open::OpenService;

use actix_web::{get, web::ServiceConfig, HttpResponse, Responder};
use serde_json::json;

pub fn configure(config: &mut ServiceConfig) {
    config.service(get_open_scores);
}

#[get("/scores")]
pub async fn get_open_scores() -> impl Responder {
    OpenService::get_open_scores().await.map_or_else(
        |_| HttpResponse::InternalServerError().body("Error fetching open scores"),
        |scores| HttpResponse::Ok().json(json!(scores)),
    )
}
