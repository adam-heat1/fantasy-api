use crate::data::{data_client::DataClient, models::open_score::OpenScore};
use chrono::{DateTime, Utc};
use chrono_tz::US::Pacific;
use sqlx::{postgres::PgRow, Error, Row};

pub struct OpenRepository;

impl OpenRepository {
    pub async fn fetch_open_scores(id: i64) -> Result<OpenScore, Error> {
        let pool = DataClient::connect().await?;

        let res = sqlx::query(
            "
            SELECT 
                id,
                labels,
                men_data,
                women_data,
                label,
                last_updated
            FROM 
                open_scores
            WHERE
                id = $1
            ",
        )
        .bind(id)
        .map(|row: PgRow| {
            let date: DateTime<Utc> = row.get("last_updated");

            OpenScore {
                id: row.get("id"),
                labels: row.get::<Vec<String>, _>("labels"),
                men_data: row.get("men_data"),
                women_data: row.get("women_data"),
                label: row.get("label"),
                last_updated: date
                    .with_timezone(&Pacific)
                    .format("%b %e %Y %I:%M %p PST")
                    .to_string(),
            }
        })
        .fetch_one(&pool)
        .await?;

        return Ok(res);
    }
}
