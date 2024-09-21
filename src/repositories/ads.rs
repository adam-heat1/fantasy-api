use crate::{data::data_client::DataClient, handlers::ads::response_models::AdResponse};
use sqlx::{Error, Row};

pub struct AdRepository;

impl AdRepository {
    pub async fn fetch_active_ads() -> Result<Vec<AdResponse>, Error> {
        let pool = DataClient::connect().await?;

        let res = sqlx::query(
            "
            SELECT
                leaderboard_url,
                banner_url,
                card_url,
                redirect_url
            FROM
                ads
            WHERE
                is_active = true
            ORDER BY
                ordinal
            ",
        )
        .map(|row: sqlx::postgres::PgRow| AdResponse {
            leaderboard_url: row.get("leaderboard_url"),
            banner_url: row.get("banner_url"),
            card_url: row.get("card_url"),
            lightbox_url: "https://assets.heat1.app/ads/gowodLightbox.jpg"
                .to_string(),
            redirect_url: row.get("redirect_url"),
        })
        .fetch_all(&pool)
        .await?;

        return Ok(res);
    }
}
