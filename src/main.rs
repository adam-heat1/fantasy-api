use actix_web::{web::scope, App, HttpServer};
use dotenv::dotenv;
use fantasy_api::handlers::{
    account::handlers as account_handlers, league::handlers as league_handlers,
    news::handlers as news_handlers,
};
use log::info;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    info!("Starting server on 8080");

    HttpServer::new(|| {
        App::new()
            .service(scope("/account/v1").configure(account_handlers::configure))
            .service(scope("/league/v1").configure(league_handlers::configure))
            .service(scope("/news/v1").configure(news_handlers::configure))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
