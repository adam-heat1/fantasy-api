use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct NewCompetitionCompetitor {
    pub id: i64,
    pub gender: String,
    #[serde(rename = "firstName")]
    pub first_name: String,
    #[serde(rename = "lastName")]
    pub last_name: String,
    pub region: String,
    pub division: String,
    pub age: i64,
    pub height: Option<String>,
    pub weight: Option<String>,
    #[serde(rename = "profileUrl")]
    pub profile_url: String,
    #[serde(rename = "crossfitId")]
    pub crossfit_id: i64,
    pub instagram: Option<String>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ActiveCompetition {
    #[serde(rename = "competitionId")]
    pub competition_id: i64,
    pub competition: String,
    #[serde(rename = "isActive")]
    pub is_active: bool,
    #[serde(rename = "isComplete")]
    pub is_complete: bool,
    pub logo: String,
    #[serde(rename = "logoDark")]
    pub logo_dark: String,
    pub date: DateTime<Utc>,
    #[serde(rename = "heat1Leagues")]
    pub heat1_leagues: Vec<i64>,
    #[serde(rename = "menCutLine")]
    pub men_cut_line: Option<i64>,
    #[serde(rename = "womenCutLine")]
    pub women_cut_line: Option<i64>,
}
