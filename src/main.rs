use actix_web::{App, HttpServer};
use dotenv::dotenv;
use fantasy_api::handlers;
use log::info;
use handlers::email::get_email_by_username;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    info!("Starting server on 8080");


    HttpServer::new(|| {
        App::new()
            // "/email?username=xxx"
            .service(get_email_by_username)
            // "/echo"
            // .service(handlers::echo)
            // .route("/email", web::get().to(get_email_by_username()))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}