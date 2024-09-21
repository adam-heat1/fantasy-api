use crate::{
    data::constants::ntfy,
    handlers::ads::response_models::{AdResponse, RotatingAdResponse},
    services::ads::AdService,
    utils::notification::spawn_notification,
};
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
    AdService::get_active_ads().await.map_or_else(
        |e| {
            if e.to_string().to_lowercase().contains("no rows returned") {
                return HttpResponse::NotFound().body("No ads found");
            }
            let error_message = format!("Error fetching ads: {:?}", e);
            spawn_notification(ntfy::ERROR.to_string(), error_message);

            HttpResponse::InternalServerError().body("Error fetching ads")
        },
        |ads| HttpResponse::Ok().json(RotatingAdResponse { ads }),
    )
    // let ads = RotatingAdResponse {
    //     ads: vec![
    //         // GOWOD - ALWAYS
    //         AdResponse {
    //             leaderboard_url: "https://heat1storage.blob.core.windows.net/app-ads/Banner.jpg"
    //                 .to_string(),
    //             banner_url: "https://heat1storage.blob.core.windows.net/app-ads/LargeBanner.jpg"
    //                 .to_string(),
    //             card_url: "https://heat1storage.blob.core.windows.net/app-ads/MediumCard.jpg"
    //                 .to_string(),
    //             lightbox_url: "https://heat1storage.blob.core.windows.net/app-ads/Lightbox.jpg"
    //                 .to_string(),
    //             redirect_url: "https://onelink.to/gowod".to_string(),
    //         },
    //         // // Ice Barrel - Event 2
    //         // AdResponse {
    //         //     leaderboard_url:
    //         //         "https://assets.heat1.app/ads/iceBarrelLeaderboard.jpg"
    //         //             .to_string(),
    //         //     banner_url:
    //         //         "https://assets.heat1.app/ads/iceBarrelBanner.jpg"
    //         //             .to_string(),
    //         //     card_url: "https://assets.heat1.app/ads/iceBarrelCard.jpg"
    //         //         .to_string(),
    //         //     lightbox_url: "https://heat1storage.blob.core.windows.net/app-ads/Lightbox.jpg"
    //         //         .to_string(),
    //         //     redirect_url: "".to_string(),
    //         // },
    //         // // Spacer Mobility - Event 3
    //         // AdResponse {
    //         //     leaderboard_url: "".to_string(),
    //         //     banner_url: "".to_string(),
    //         //     card_url: "".to_string(),
    //         //     lightbox_url: "https://heat1storage.blob.core.windows.net/app-ads/Lightbox.jpg"
    //         //         .to_string(),
    //         //     redirect_url: "".to_string(),
    //         // },
    //         // Strong - Event 4
    //         // AdResponse {
    //         //     leaderboard_url:
    //         //         "https://assets.heat1.app/ads/strongLeaderboard.png"
    //         //             .to_string(),
    //         //     banner_url: "https://assets.heat1.app/ads/strongBanner.png"
    //         //         .to_string(),
    //         //     card_url: "https://assets.heat1.app/ads/strongCard.png"
    //         //         .to_string(),
    //         //     lightbox_url: "https://heat1storage.blob.core.windows.net/app-ads/Lightbox.jpg"
    //         //         .to_string(),
    //         //     redirect_url: "https://strongcoffeecompany.com/pages/rodeo-club".to_string(),
    //         // },
    //         // // BHTD - Event 5
    //         AdResponse {
    //             leaderboard_url:
    //                 "https://assets.heat1.app/ads/bhtdLeaderboard.png"
    //                     .to_string(),
    //             banner_url: "https://assets.heat1.app/ads/bhtdBanner.png"
    //                 .to_string(),
    //             card_url: "https://assets.heat1.app/ads/bhtdCard.png"
    //                 .to_string(),
    //             lightbox_url: "https://heat1storage.blob.core.windows.net/app-ads/Lightbox.jpg"
    //                 .to_string(),
    //             redirect_url: "https://www.besthouroftheirday.com/".to_string(),
    //         },
    //         // // UNKOWN - Event 6
    //         // AdResponse {
    //         //     leaderboard_url: "".to_string(),
    //         //     banner_url: "".to_string(),
    //         //     card_url: "".to_string(),
    //         //     lightbox_url: "https://heat1storage.blob.core.windows.net/app-ads/Lightbox.jpg"
    //         //         .to_string(),
    //         //     redirect_url: "".to_string(),
    //         // },
    //         // // JOCKO FUEL - Event 7
    //         // AdResponse {
    //         //     leaderboard_url: "https://assets.heat1.app/ads/jockoLeaderboard.jpg".to_string(),
    //         //     banner_url: "https://assets.heat1.app/ads/jockoBanner.jpg".to_string(),
    //         //     card_url: "https://assets.heat1.app/ads/jockoCard.jpg".to_string(),
    //         //     lightbox_url: "https://heat1storage.blob.core.windows.net/app-ads/Lightbox.jpg"
    //         //         .to_string(),
    //         //     redirect_url: "https://jockofuel.com/pages/crossfit%20?utm_source=CF_LIVESTREAM_Aug2024&utm_medium=CF_LIVESTREAM_Aug2024&utm_campaign=JF_CFGames2024_5KGiveaway&utm_id=CF_LIVESTREAM_Aug2024".to_string(),
    //         // },
    //         // // 2POOD - Event 8
    //         // AdResponse {
    //         //     leaderboard_url: "https://assets.heat1.app/ads/2poodLeaderboard.png".to_string(),
    //         //     banner_url: "https://assets.heat1.app/ads/2poodBanner.png".to_string(),
    //         //     card_url: "https://assets.heat1.app/ads/2poodCard.png".to_string(),
    //         //     lightbox_url: "https://heat1storage.blob.core.windows.net/app-ads/Lightbox.jpg"
    //         //         .to_string(),
    //         //     redirect_url: "2pood.com/gamessale".to_string(),
    //         // },
    //         // // ESC Sounds - Event 9
    //         // AdResponse {
    //         //     leaderboard_url: "https://assets.heat1.app/ads/escLeaderboard.jpg".to_string(),
    //         //     banner_url: "https://assets.heat1.app/ads/escBanner.jpg".to_string(),
    //         //     card_url: "https://assets.heat1.app/ads/escCard.jpg".to_string(),
    //         //     lightbox_url: "https://heat1storage.blob.core.windows.net/app-ads/Lightbox.jpg"
    //         //         .to_string(),
    //         //     redirect_url: "".to_string(),
    //         // },
    //         // // ESC Sounds DB - Event 9
    //         // AdResponse {
    //         //     leaderboard_url: "https://assets.heat1.app/ads/escDbLeaderboard.jpg".to_string(),
    //         //     banner_url: "https://assets.heat1.app/ads/escDbBanner.jpg".to_string(),
    //         //     card_url: "https://assets.heat1.app/ads/escDbCard.jpg".to_string(),
    //         //     lightbox_url: "https://heat1storage.blob.core.windows.net/app-ads/Lightbox.jpg"
    //         //         .to_string(),
    //         //     redirect_url: "".to_string(),
    //         // },
    //         // // RXSG - Event 10
    //         // AdResponse {
    //         //     leaderboard_url: "".to_string(),
    //         //     banner_url: "".to_string(),
    //         //     card_url: "".to_string(),
    //         //     lightbox_url: "https://heat1storage.blob.core.windows.net/app-ads/Lightbox.jpg"
    //         //         .to_string(),
    //         //     redirect_url: "".to_string(),
    //         // },
    //         // // Airwaav - Event 11
    //         // AdResponse {
    //         //     leaderboard_url: "".to_string(),
    //         //     banner_url: "".to_string(),
    //         //     card_url: "".to_string(),
    //         //     lightbox_url: "https://heat1storage.blob.core.windows.net/app-ads/Lightbox.jpg"
    //         //         .to_string(),
    //         //     redirect_url: "".to_string(),
    //         // },
    //     ],
    // };
    // HttpResponse::Ok().json(ads)
}
