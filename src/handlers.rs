use log::debug;
use tide::{Body, Request, Response, StatusCode};

pub(crate) async fn get_health(_: Request<()>) -> tide::Result {
    let success_message = "Healthy!";
    debug!("GET /health");
    let mut res = Response::new(StatusCode::Ok);
    res.set_body(Body::from_string(success_message.to_string()));
    Ok(res)
}
