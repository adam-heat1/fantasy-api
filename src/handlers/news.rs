use std::env;

use crate::services::news::NewsService;
use actix_web::{
    get, post,
    web::{Json, ServiceConfig},
    HttpResponse, Responder,
};
use serde_derive::Deserialize;

pub fn configure(config: &mut ServiceConfig) {
    config.service(get_news).service(create_news_blurb);
}

#[get("/")]
pub(crate) async fn get_news() -> impl Responder {
    NewsService::get_news().await.map_or_else(
        |e| {
            log::error!("Error getting news: -> {:?}", e);
            HttpResponse::InternalServerError().body("Error creating article!")
        },
        |_| HttpResponse::Ok().finish(),
    )
}

#[derive(Deserialize, Clone, Debug)]
pub struct CreateNewsBlurbViewModel {
    pub source: String,
    pub title: String,
    pub description: String,
    pub link: String,
    pub date: String,
}

#[post("/")]
pub(crate) async fn create_news_blurb(body: Json<CreateNewsBlurbViewModel>) -> impl Responder {
    if body.source.trim().is_empty() {
        return HttpResponse::BadRequest().body("No source provided!");
    }
    if body.title.trim().is_empty() {
        return HttpResponse::BadRequest().body("No title provided!");
    }
    if body.description.trim().is_empty() {
        return HttpResponse::BadRequest().body("No description provided!");
    }
    if body.link.trim().is_empty() {
        return HttpResponse::BadRequest().body("No link provided!");
    }
    if body.date.trim().is_empty() {
        return HttpResponse::BadRequest().body("No date provided!");
    }

    let article: CreateNewsBlurbViewModel = body.into_inner();

    NewsService::create_article(article.clone())
        .await
        .map_or_else(
            |e| {
                let error_message = format!("Error creating account: {:?}: -> {:?}", article, e);
                log::error!("Error creating account: {:?}: -> {:?}", article, e);
                let unknown_error_provider =
                    env::var("NTFY_UNKNOWN_ERROR").expect("NTFY_UNKNOWN_ERROR must be set");
                let client = reqwest::blocking::Client::new();
                let _ = client
                    .post(format!("ntfy.sh/{}", unknown_error_provider))
                    .body(error_message)
                    .send();
                HttpResponse::InternalServerError().body("Error creating article!")
            },
            |_| HttpResponse::Ok().finish(),
        )
}
