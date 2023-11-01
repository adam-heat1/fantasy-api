use tide::Server;

use crate::handlers;

pub(crate) async fn get_app() -> tide::Result<Server<()>> {
    let mut app = tide::new();

    app.at("/healthz").get(handlers::get_health);

    Ok(app)
}
