use std::{env, fmt};

use crate::{
    data::models::news::News, handlers::news::CreateNewsBlurbViewModel,
    repositories::news::NewsRepository,
};
use sqlx::Error;

pub struct NewsService;

impl NewsService {
    pub async fn get_news() -> Result<Vec<News>, Error> {
        NewsRepository::fetch_articles().await
    }

    pub async fn create_article(article: CreateNewsBlurbViewModel) -> Result<(), Error> {
        let src = &article.source;

        let news = News {
            id: 0,
            title: article.title,
            description: article.description,
            link: article.link,
            header: Self::get_header_by_source(src).await,
            date: article.date,
            label: "".to_string(),
            image_url: Self::get_image_url_by_source(src),
        };

        NewsRepository::insert_article(news).await
    }

    async fn get_header_by_source(source: &str) -> String {
        match source.to_lowercase().trim() {
            "barbell spin" | "the barbell spin" => "The Barbell Spin".to_string(),
            "heat 1" => "Heat 1".to_string(),
            "coffee pods & wods" | "cpw" | "coffee pods and wods" => {
                "Coffee Pods & Wods".to_string()
            }
            "mayhem" | "crossfit mayhem" => "Mayhem".to_string(),
            "shut up and scribble" | "shut up & scribble" => "Shut Up & Scribble".to_string(),
            "talking elite fitness" | "tef" => "Talking Elite Fitness".to_string(),
            "the sevan podcast" | "sevan" | "sevan podcast" => "The Sevan Podcast".to_string(),
            "b.friendly fitness" | "bfriendly fitness" | "bfriendly" | "brian friend" => {
                "B.Friendly Fitness".to_string()
            }
            "pfaa" => "PFAA".to_string(),
            "training think tank" | "ttt" => "Training Think Tank".to_string(),
            "dave castro" | "castro" | "tdc" | "the dave castro" => "Dave Castro".to_string(),
            "get with the programming" | "gwtp" => {
                "Get With The Programming".to_string().to_string()
            }
            "btwb" | "beyond the whiteboard" => "Beyond The Whiteboard".to_string(),
            "josh bridges" => "Josh Bridges".to_string(),
            "prvn" | "prvn fitness" => "PRVN".to_string(),
            "krypton" | "crossfit krypton" => "Krypton".to_string(),
            _ => {
                let unknown_media_provider =
                    env::var("NTFY_UNKNOWN_MEDIA").expect("NTFY_UNKNOWN_MEDIA must be set");
                let client = reqwest::Client::new();
                let _ = client
                    .post(format!("ntfy.sh/{}", unknown_media_provider))
                    .body(format!("Media provider {} not found", source))
                    .send()
                    .await;
                return "".to_string();
            }
        }
    }

    fn get_image_url_by_source(source: &str) -> String {
        match source.to_lowercase().trim() {
            "barbell spin" | "the barbell spin" => {
                "https://heat1storage.blob.core.windows.net/user/24.jpg".to_string()
            }
            "heat 1" => "https://heat1storage.blob.core.windows.net/logo/logo.png".to_string(),
            "coffee pods & wods" | "cpw" | "coffee pods and wods" => {
                "https://heat1storage.blob.core.windows.net/user/196.jpg".to_string()
            }
            "mayhem" | "crossfit mayhem" => {
                "https://heat1storage.blob.core.windows.net/media/mayhem.jpg".to_string()
            }
            "shut up and scribble" | "shut up & scribble" => {
                "https://heat1storage.blob.core.windows.net/media/shutupandscribble.jpg".to_string()
            }
            "talking elite fitness" | "tef" => {
                "https://heat1storage.blob.core.windows.net/media/talkingelitefitness.jpg"
                    .to_string()
            }
            "the sevan podcast" | "sevan" | "sevan podcast" => {
                "https://heat1storage.blob.core.windows.net/user/1814.jpg".to_string()
            }
            "b.friendly fitness" | "bfriendly fitness" | "bfriendly" | "brian friend" => {
                "https://heat1storage.blob.core.windows.net/user/1255.jpg".to_string()
            }
            "pfaa" => "https://heat1storage.blob.core.windows.net/media/pfaa.jpg".to_string(),
            "training think tank" | "ttt" => {
                "https://heat1storage.blob.core.windows.net/media/ttt.jpg".to_string()
            }
            "dave castro" | "castro" | "tdc" | "the dave castro" => {
                "https://heat1storage.blob.core.windows.net/media/davecastro.jpg".to_string()
            }
            "get with the programming" | "gwtp" => {
                "https://heat1storage.blob.core.windows.net/media/getwiththeprogramming.jpg"
                    .to_string()
            }
            "btwb" | "beyond the whiteboard" => {
                "https://heat1storage.blob.core.windows.net/media/btwb.jpg".to_string()
            }
            "josh bridges" => {
                "https://heat1storage.blob.core.windows.net/media/joshbridges.jpg".to_string()
            }
            "prvn" | "prvn fitness" => {
                "https://heat1storage.blob.core.windows.net/media/prvn.jpg".to_string()
            }
            "krypton" | "crossfit krypton" => {
                "https://heat1storage.blob.core.windows.net/media/krypton.jpg".to_string()
            }
            _ => "".to_string(),
        }
    }
}
