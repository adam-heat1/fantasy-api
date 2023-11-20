use actix_web::{web::scope, App, HttpServer};
use dotenv::dotenv;
use fantasy_api::handlers::{account, news};
use log::info;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    info!("Starting server on 8080");

    HttpServer::new(|| {
        App::new()
            .service(scope("/v1/account").configure(account::configure))
            .service(scope("/v1/news").configure(news::configure))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
