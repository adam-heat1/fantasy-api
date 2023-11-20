use serde::Serialize;

#[derive(Serialize)]
pub struct News {
    pub id: u64,
    pub image_url: String,
    pub title: String,
    pub description: String,
    pub header: String,
    pub link: String,
    pub label: String,
    pub date: String,
}
