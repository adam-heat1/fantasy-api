use serde::Serialize;

#[derive(Serialize)]
pub struct News {
    pub id: u64,
    pub image_url: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub header: Option<String>,
    pub link: Option<String>,
    pub label: Option<String>,
    pub date: Option<String>,
}
