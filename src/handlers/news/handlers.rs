use crate::{
    data::constants::ntfy, handlers::news::request_models::CreateNewsBlurb,
    services::news::NewsService, utils::notification::spawn_notification,
};
use actix_web::{
    get, post,
    web::{Json, ServiceConfig},
    HttpResponse, Responder,
};
use validator::Validate;

pub fn configure(config: &mut ServiceConfig) {
    config.service(get_news).service(create_news_blurb);
}

#[get("/feed")]
pub async fn get_news() -> impl Responder {
    NewsService::get_news().await.map_or_else(
        |e| {
            let message = format!("get_news: -> {:?}", e);
            spawn_notification(ntfy::ERROR.to_string(), message);

            HttpResponse::InternalServerError().body("Error getting news")
        },
        |response| HttpResponse::Ok().json(response),
    )
}

#[post("/article")]
pub async fn create_news_blurb(body: Json<CreateNewsBlurb>) -> impl Responder {
    if body.validate().is_err() {
        let message = format!("create_news_blurb: -> {:?}", body.validate().unwrap_err());
        spawn_notification(ntfy::ERROR.to_string(), message);

        return HttpResponse::BadRequest().body("Invalid update username request");
    }

    let article: CreateNewsBlurb = body.into_inner();

    NewsService::create_article(article.clone())
        .await
        .map_or_else(
            |e| {
                let error_message = format!("Error creating account: {:?}: -> {:?}", article, e);
                spawn_notification(ntfy::ERROR.to_string(), error_message);

                HttpResponse::InternalServerError().body("Error creating article!")
            },
            |_| HttpResponse::Ok().finish(),
        )
}
