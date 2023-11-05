use std::env;

use actix_web::{get, post, HttpResponse, Responder};

#[get("/")]
pub(crate) async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
pub(crate) async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

pub(crate) async fn manual_hello() -> impl Responder {
    // Test env "TARGET" which defined when `docker run`, or `gcloud run deploy --set-env-vars`
    // Depend on your platform target. (See README.md)
    let test_target = match env::var("TARGET") {
        Ok(target) => format!("Hey {target}!"),
        Err(_e) => "No TARGET env defined!".to_owned(),
    };

    // Response with test_target
    HttpResponse::Ok().body(test_target)
}
