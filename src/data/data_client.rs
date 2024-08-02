use sqlx::{postgres::PgPoolOptions, Error, Pool, Postgres};
use std::env;

pub struct DataClient;

impl DataClient {
    pub async fn connect() -> Result<Pool<Postgres>, Error> {
        // get envv aror default  to value
        // DATABASE_URL is the connection string

        let connection_string = env::var("DATABASE_URL").unwrap_or(
            "postgres://postgres:hDEEa_*Ff2ciidPEBjU2bT6p@34.170.156.72:5432/postgres".to_string(),
        );

        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&connection_string)
            .await
            .expect("Failed to connect to Postgres");

        Ok(pool)
    }
}
