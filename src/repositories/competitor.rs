use crate::data::data_client::DataClient;
use crate::handlers::athlete::response_models::CompetitionCompetitorResponse;
use sqlx::{Error, Row};

pub struct CompetitorRepository;

impl CompetitorRepository {
    pub async fn fetch_competition_competitor(
        competition_id: i64,
        competitor_id: i64,
    ) -> Result<CompetitionCompetitorResponse, Error> {
        let pool = DataClient::connect().await?;

        let res = sqlx::query(
            "
            SELECT
                c.first_name,
                c.last_name,
                c.age,
                c.height,
                c.weight,
                c.instagram,
                cc.news_blurb,
                r.name as region
            FROM
                competition_competitor as cc
            JOIN
                competitor as c
                ON cc.competitor_id = c.id
            JOIN 
                region as r
                ON c.region_id = r.id
            WHERE
                cc.competition_id = $1
                AND cc.competitor_id = $2
            ",
        )
        .bind(competition_id)
        .bind(competitor_id)
        .map(|row: sqlx::postgres::PgRow| CompetitionCompetitorResponse {
            first_name: row.get("first_name"),
            last_name: row.get("last_name"),
            age: row.get("age"),
            height: row.get("height"),
            weight: row.get("weight"),
            instagram: row.get("instagram"),
            region: row.get("region"),
            news_blurb: row.get("news_blurb"),
        })
        .fetch_one(&pool)
        .await?;

        return Ok(res);
    }
}
