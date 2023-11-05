use std::env;

use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres, Error};

pub struct DataClient;

impl DataClient {
    pub async fn connect() -> Result<Pool<Postgres>, Error> {
        dotenv().ok();

        let connection_string = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&connection_string).await?;

        Ok(pool)
    }
}
