use sqlx::{postgres::PgRow, Error, Row};

use crate::data::{data_client::DataClient, models::news::News};

pub struct NewsRepository;

impl NewsRepository {
    pub async fn fetch_articles() -> Result<Vec<News>, Error> {
        let pool = DataClient::connect().await?;

        let res = sqlx::query(
            "
            SELECT 
                id,
                image_url,
                title,
                description,
                link,
                label,
                header,
                date
            FROM 
                news
            ",
        )
        .map(|row: PgRow| {
            return News {
                id: row.get::<i64, _>("id") as u64,
                image_url: row.get("image_url"),
                title: row.get("title"),
                description: row.get("description"),
                link: row.get("link"),
                label: row.get("label"),
                header: row.get("header"),
                date: row.get("date"),
            };
        })
        .fetch_all(&pool)
        .await?;

        return Ok(res);
    }
    pub async fn insert_article(article: News) -> Result<(), Error> {
        let pool = DataClient::connect().await?;

        sqlx::query(
            "
            INSERT INTO
                news
            (title, description, image_url, link, header, date) 
            VALUES 
                ($1, $2, $3, $4, $5, $6)
            ",
        )
        .bind(article.title)
        .bind(article.description)
        .bind(article.image_url)
        .bind(article.link)
        .bind(article.header)
        .bind(article.date)
        .execute(&pool)
        .await?;

        return Ok(());
    }
}
