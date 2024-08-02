use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct AdResponse {
    #[serde(rename = "leaderboardUrl")]
    pub leaderboard_url: String,
    #[serde(rename = "bannerUrl")]
    pub banner_url: String,
    #[serde(rename = "cardUrl")]
    pub card_url: String,
    #[serde(rename = "lightboxUrl")]
    pub lightbox_url: String,
    #[serde(rename = "redirectUrl")]
    pub redirect_url: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct RotatingAdResponse {
    pub ads: Vec<AdResponse>,
}
