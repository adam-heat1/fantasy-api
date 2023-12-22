use serde_derive::Serialize;

#[derive(Serialize, Clone, Debug)]
pub struct CompetitionCompetitorResponse {
    #[serde(rename = "firstName")]
    pub first_name: String,
    #[serde(rename = "lastName")]
    pub last_name: String,
    pub region: String,
    pub instagram: Option<String>,
    #[serde(rename = "newsBlurb")]
    pub news_blurb: Option<String>,
    pub age: Option<i64>,
    pub height: Option<String>,
    pub weight: Option<String>,
}
