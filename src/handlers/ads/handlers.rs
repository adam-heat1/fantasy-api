use crate::handlers::ads::response_models::{AdResponse, RotatingAdResponse};
use actix_web::{get, web::ServiceConfig, HttpResponse, Responder};

pub fn configure(config: &mut ServiceConfig) {
    config.service(get_pliability_ads).service(get_ads);
}

#[get("/pliability")]
pub async fn get_pliability_ads() -> impl Responder {
    let ads = AdResponse {
        leaderboard_url: "https://heat1storage.blob.core.windows.net/app-ads/Banner.jpg"
            .to_string(),
        banner_url: "https://heat1storage.blob.core.windows.net/app-ads/LargeBanner.jpg"
            .to_string(),
        card_url: "https://heat1storage.blob.core.windows.net/app-ads/MediumCard.jpg".to_string(),
        lightbox_url: "https://heat1storage.blob.core.windows.net/app-ads/Lightbox.jpg".to_string(),
        redirect_url: "https://onelink.to/gowod".to_string(),
    };
    HttpResponse::Ok().json(ads)
}

#[get("/")]
pub async fn get_ads() -> impl Responder {
    let ads = RotatingAdResponse {
        ads: vec![
            AdResponse {
                leaderboard_url: "https://heat1storage.blob.core.windows.net/app-ads/Banner.jpg"
                    .to_string(),
                banner_url: "https://heat1storage.blob.core.windows.net/app-ads/LargeBanner.jpg"
                    .to_string(),
                card_url: "https://heat1storage.blob.core.windows.net/app-ads/MediumCard.jpg"
                    .to_string(),
                lightbox_url: "https://heat1storage.blob.core.windows.net/app-ads/Lightbox.jpg"
                    .to_string(),
                redirect_url: "https://onelink.to/gowod".to_string(),
            },
            AdResponse {
                leaderboard_url: "https://heat1storage.blob.core.windows.net/app-ads/Banner.jpg"
                    .to_string(),
                banner_url: "https://heat1storage.blob.core.windows.net/app-ads/LargeBanner.jpg"
                    .to_string(),
                card_url: "https://heat1storage.blob.core.windows.net/app-ads/MediumCard.jpg"
                    .to_string(),
                lightbox_url: "https://heat1storage.blob.core.windows.net/app-ads/Lightbox.jpg"
                    .to_string(),
                redirect_url: "https://onelink.to/gowod".to_string(),
            },
        ],
    };
    HttpResponse::Ok().json(ads)
}
