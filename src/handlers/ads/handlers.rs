use crate::handlers::ads::response_models::AdResponse;
use actix_web::{get, web::ServiceConfig, HttpResponse, Responder};

pub fn configure(config: &mut ServiceConfig) {
    config.service(get_pliability_ads);
}

#[get("/pliability")]
pub async fn get_pliability_ads() -> impl Responder {
    let ads = AdResponse{
    leaderboard_url: "https://media.pliability.com/image/upload/v1704939325/vendor/heat1/pliability_heat1-banner.png".to_string(),
    banner_url: "https://media.pliability.com/image/upload/v1704854839/vendor/heat1/pliability_heat1-large-banner.png".to_string(),
    card_url: "https://media.pliability.com/image/upload/v1704854841/vendor/heat1/pliability_heat1-medium-card.png".to_string(),
    lightbox_url: "https://media.pliability.com/image/upload/v1704854838/vendor/heat1/pliability_heat1-lightbox.png".to_string(),
    redirect_url: "https://pliability.app/event-signup".to_string()
};
    HttpResponse::Ok().json(ads)
}
