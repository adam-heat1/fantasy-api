mod handlers;

use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use log::{info};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    info!("Starting server on 8080");


    HttpServer::new(|| {
        App::new()
            // "/"
            .service(handlers::hello)
            // "/echo"
            .service(handlers::echo)
            // "/hey"
            .route("/hey", web::get().to(handlers::manual_hello))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}