use crate::data::data_client::DataClient;
use crate::handlers::athlete::response_models::CompetitionCompetitorResponse;
use crate::handlers::competition::response_models::NewCompetitionCompetitor;
use sqlx::{Error, Row};

pub struct CompetitorRepository;

impl CompetitorRepository {
    pub async fn fetch_competitor(name: String) -> Result<Vec<NewCompetitionCompetitor>, Error> {
        let pool = DataClient::connect().await?;

        let res = sqlx::query(
            "
            SELECT
                c.id,
                c.first_name,
                c.last_name,
                c.age,
                c.height,
                c.weight,
                c.profile_url,
                c.crossfit_id,
                c.instagram,
                co.name as country,
                d.name as division,
                r.name as region,
                g.name as gender
            FROM competitor as c
            JOIN
                region as r
                ON c.region_id = r.id
            JOIN
                country as co
                ON co.id = c.country_id
            JOIN
                division as d
                ON d.id = c.division_id
            JOIN
                gender as g
                ON g.id = c.gender_id
            WHERE
                lower(CONCAT(c.first_name, ' ', c.last_name)) like $1
            ",
        )
        .bind(format!("%{name}%").to_lowercase())
        .map(|row: sqlx::postgres::PgRow| NewCompetitionCompetitor {
            id: row.get("id"),
            gender: row.get("gender"),
            first_name: row.get("first_name"),
            last_name: row.get("last_name"),
            region: row.get("region"),
            division: row.get("division"),
            age: row.get("age"),
            height: row.get("height"),
            weight: row.get("weight"),
            profile_url: row.get("profile_url"),
            crossfit_id: row.get("crossfit_id"),
            instagram: row.get("instagram"),
        })
        .fetch_all(&pool)
        .await?;

        return Ok(res);
    }

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

    pub async fn create_competition_competitor(
        competiton_id: i64,
        competitor_id: i64,
    ) -> Result<(), Error> {
        let pool = DataClient::connect().await?;

        let _ = sqlx::query(
            "
            INSERT INTO competition_competitor (competition_id, competitor_id)
            VALUES ($1, $2)
            ",
        )
        .bind(competiton_id)
        .bind(competitor_id)
        .execute(&pool)
        .await?;

        Ok(())
    }
}
