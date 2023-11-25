use crate::{
    data::{models::news::News, constants::ntfy}, handlers::news::CreateNewsBlurbViewModel,
    utils::notification::spawn_notification,
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
            title: Some(article.title),
            description: Some(article.description),
            link: Some(article.link),
            header: Some(Self::get_header_by_source(src).to_string()),
            date: Some(article.date),
            label: None,
            image_url: Some(Self::get_image_url_by_source(src).to_string()),
        };

        NewsRepository::insert_article(news).await
    }

    fn get_header_by_source(source: &str) -> &str {
        match source.to_lowercase().trim() {
            "barbell spin" | "the barbell spin" => "The Barbell Spin",
            "heat 1" => "Heat 1",
            "coffee pods & wods" | "cpw" | "coffee pods and wods" => {
                "Coffee Pods & Wods"
            }
            "mayhem" | "crossfit mayhem" => "Mayhem",
            "shut up and scribble" | "shut up & scribble" => "Shut Up & Scribble",
            "talking elite fitness" | "tef" => "Talking Elite Fitness",
            "the sevan podcast" | "sevan" | "sevan podcast" => "The Sevan Podcast",
            "b.friendly fitness" | "bfriendly fitness" | "bfriendly" | "brian friend" => {
                "B.Friendly Fitness"
            }
            "pfaa" => "PFAA",
            "training think tank" | "ttt" => "Training Think Tank",
            "dave castro" | "castro" | "tdc" | "the dave castro" => "Dave Castro",
            "get with the programming" | "gwtp" => {
                "Get With The Programming"
            }
            "btwb" | "beyond the whiteboard" => "Beyond The Whiteboard",
            "josh bridges" => "Josh Bridges",
            "prvn" | "prvn fitness" => "PRVN",
            "krypton" | "crossfit krypton" => "Krypton",
            _ => {
                let message = format!("Media provider {} not found", source);
                spawn_notification(ntfy::MEDIA.to_string(), message);
                return "";
            }
        }
    }

    fn get_image_url_by_source(source: &str) -> &str {
        match source.to_lowercase().trim() {
            "barbell spin" | "the barbell spin" => {
                "https://heat1storage.blob.core.windows.net/user/24.jpg"
            }
            "heat 1" => "https://heat1storage.blob.core.windows.net/logo/logo.png",
            "coffee pods & wods" | "cpw" | "coffee pods and wods" => {
                "https://heat1storage.blob.core.windows.net/user/196.jpg"
            }
            "mayhem" | "crossfit mayhem" => {
                "https://heat1storage.blob.core.windows.net/media/mayhem.jpg"
            }
            "shut up and scribble" | "shut up & scribble" => {
                "https://heat1storage.blob.core.windows.net/media/shutupandscribble.jpg"
            }
            "talking elite fitness" | "tef" => {
                "https://heat1storage.blob.core.windows.net/media/talkingelitefitness.jpg"

            }
            "the sevan podcast" | "sevan" | "sevan podcast" => {
                "https://heat1storage.blob.core.windows.net/user/1814.jpg"
            }
            "b.friendly fitness" | "bfriendly fitness" | "bfriendly" | "brian friend" => {
                "https://heat1storage.blob.core.windows.net/user/1255.jpg"
            }
            "pfaa" => "https://heat1storage.blob.core.windows.net/media/pfaa.jpg",
            "training think tank" | "ttt" => {
                "https://heat1storage.blob.core.windows.net/media/ttt.jpg"
            }
            "dave castro" | "castro" | "tdc" | "the dave castro" => {
                "https://heat1storage.blob.core.windows.net/media/davecastro.jpg"
            }
            "get with the programming" | "gwtp" => {
                "https://heat1storage.blob.core.windows.net/media/getwiththeprogramming.jpg"

            }
            "btwb" | "beyond the whiteboard" => {
                "https://heat1storage.blob.core.windows.net/media/btwb.jpg"
            }
            "josh bridges" => {
                "https://heat1storage.blob.core.windows.net/media/joshbridges.jpg"
            }
            "prvn" | "prvn fitness" => {
                "https://heat1storage.blob.core.windows.net/media/prvn.jpg"
            }
            "krypton" | "crossfit krypton" => {
                "https://heat1storage.blob.core.windows.net/media/krypton.jpg"
            }
            _ => "",
        }
    }
}
