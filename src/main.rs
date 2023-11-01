use async_std::task;
use dotenv::dotenv;

mod error_response;
mod handlers;
mod server;

use server::get_app;

fn main() -> tide::Result<()> {
    task::block_on(async {
        dotenv().ok();

        let app = get_app().await?;

        app.listen("0.0.0.0:3030").await?;
        Ok(())
    })
}
