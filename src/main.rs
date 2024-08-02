use actix_cors::Cors;
use actix_web::{web::scope, App, HttpServer};
use fantasy_api::handlers::{
    account::handlers as account_handlers, ads::handlers as ad_handlers,
    athlete::handlers as athlete_handlers, competition::handlers as competition_handlers,
    crossfit::handlers as crossfit_handlers, league::handlers as league_handlers,
    news::handlers as news_handlers, open::handlers as open_handlers,
    props::handlers as prop_handlers,
};
use log::info;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    info!("Starting server on 8080");

    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        // opentelemetry_otlp::new_pipeline().tracing()
        //
        // let honeycomb_key =
        //     "hcaik_01hxkzhpc7th9z8hb1yj472cmhyrsq6cw0s18n3yz78speskkqyckf2dgb".to_string();
        // let honeycomb_urll = "https://api.honeycomb.io/".to_string();
        // let honeycomb_service = "your-service-name";
        App::new()
            .wrap(cors)
            .service(scope("/account/v1").configure(account_handlers::configure))
            .service(scope("/athlete/v1").configure(athlete_handlers::configure))
            .service(scope("/competition/v1").configure(competition_handlers::configure))
            .service(scope("/league/v1").configure(league_handlers::configure))
            .service(scope("/news/v1").configure(news_handlers::configure))
            .service(scope("/ads/v1").configure(ad_handlers::configure))
            .service(scope("/props/v1").configure(prop_handlers::configure))
            .service(scope("/crossfit/v1").configure(crossfit_handlers::configure))
            .service(scope("/open/v1").configure(open_handlers::configure))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
