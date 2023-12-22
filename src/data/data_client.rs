use sqlx::{postgres::PgPoolOptions, Error, Pool, Postgres};
use std::env;

pub struct DataClient;

impl DataClient {
    pub async fn connect() -> Result<Pool<Postgres>, Error> {
        let connection_string = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&connection_string)
            .await
            .expect("Failed to connect to Postgres");

        Ok(pool)
    }
}
