use crate::handlers::competition::response_models::{ActiveCompetition, NewCompetitionCompetitor};
use crate::repositories::competitor::CompetitorRepository;
use chrono::{TimeZone, Utc};
use sqlx::Error;

pub struct CompetitionService;

impl CompetitionService {
    pub async fn fetch_active_beta_competitions() -> Result<Vec<ActiveCompetition>, Error> {
        let d = Utc.with_ymd_and_hms(2024, 5, 16, 12, 0, 0).unwrap();
        println!("{}", d);
        let response: Vec<ActiveCompetition> = vec![ActiveCompetition {
            competition_id: 28,
            competition: "CrossFit Games".to_string(),
            is_active: false,
            is_complete: false,
            logo: "https://heat1storage.blob.core.windows.net/competition/crossfitGames2.jpg"
                .to_string(),
            logo_dark: "https://heat1storage.blob.core.windows.net/competition/crossfitGames2.jpg"
                .to_string(),
            date: Utc.with_ymd_and_hms(2024, 8, 01, 12, 0, 0).unwrap(),
            heat1_leagues: vec![759, 765],
            men_cut_line: None,
            women_cut_line: None,
        }];

        Ok(response)
    }

    pub async fn fetch_active_competitions() -> Result<Vec<ActiveCompetition>, Error> {
        let d = Utc.with_ymd_and_hms(2024, 5, 16, 12, 0, 0).unwrap();
        println!("{}", d);
        let response: Vec<ActiveCompetition> = vec![
            ActiveCompetition {
                competition_id: 22,
                competition: "Europe".to_string(),
                is_active: false,
                is_complete: false,
                logo: "https://heat1storage.blob.core.windows.net/competition/europe.png"
                    .to_string(),
                logo_dark: "https://heat1storage.blob.core.windows.net/competition/europeWhite.png"
                    .to_string(),
                date: Utc.with_ymd_and_hms(2024, 5, 17, 12, 0, 0).unwrap(),
                heat1_leagues: vec![514, 515],
                men_cut_line: Some(10i64),
                women_cut_line: Some(10i64),
            },
            ActiveCompetition {
                competition_id: 21,
                competition: "Asia".to_string(),
                is_active: false,
                is_complete: false,
                logo: "https://heat1storage.blob.core.windows.net/competition/asia.png".to_string(),
                logo_dark: "https://heat1storage.blob.core.windows.net/competition/asiaWhite.png".to_string(),
                date: Utc.with_ymd_and_hms(2024, 5, 16, 12, 0, 0).unwrap(),
                heat1_leagues: vec![512, 513],
                men_cut_line: Some(2i64),
                women_cut_line: Some(3i64),
            },
            ActiveCompetition {
                competition_id: 24,
                competition: "NA West".to_string(),
                is_active: false,
                is_complete: false,
                logo: "https://heat1storage.blob.core.windows.net/competition/northAmericaWest.png"
                    .to_string(),
                logo_dark: "https://heat1storage.blob.core.windows.net/competition/NorthAmericaWestWhite.png"
                    .to_string(),
                date: Utc.with_ymd_and_hms(2024, 5, 24, 12, 0, 0).unwrap(),
                heat1_leagues: vec![518, 519],
                men_cut_line: Some(9i64),
                women_cut_line: Some(8i64),
            },
            ActiveCompetition {
                competition_id: 23,
                competition: "Oceania".to_string(),
                is_active: false,
                is_complete: false,
                logo: "https://heat1storage.blob.core.windows.net/competition/oceania.png"
                    .to_string(),
                logo_dark: "https://heat1storage.blob.core.windows.net/competition/OceaniaWhite.png"
                    .to_string(),
                date: Utc.with_ymd_and_hms(2024, 5, 23, 12, 0, 0).unwrap(),
                heat1_leagues: vec![516, 517],
                men_cut_line: Some(4i64),
                women_cut_line: Some(4i64),
            },
            ActiveCompetition {
                competition_id: 27,
                competition: "NA East".to_string(),
                is_active: false,
                is_complete: false,
                logo: "https://heat1storage.blob.core.windows.net/competition/northAmericaEast.png"
                    .to_string(),
                logo_dark: "https://heat1storage.blob.core.windows.net/competition/NorthAmericaEastWhite.png"
                    .to_string(),
                date: Utc.with_ymd_and_hms(2024, 5, 31, 12, 0, 0).unwrap(),
                heat1_leagues: vec![524, 525],
                men_cut_line: Some(11i64),
                women_cut_line: Some(11i64),
            },
            ActiveCompetition {
                competition_id: 25,
                competition: "Africa".to_string(),
                is_active: false,
                is_complete: false,
                logo: "https://heat1storage.blob.core.windows.net/competition/africa.png"
                    .to_string(),
                logo_dark: "https://heat1storage.blob.core.windows.net/competition/africaWhite.png"
                    .to_string(),
                date: Utc.with_ymd_and_hms(2024, 5, 31, 12, 0, 0).unwrap(),
                heat1_leagues: vec![520, 521],
                men_cut_line: Some(1i64),
                women_cut_line: Some(1i64),
            },
            ActiveCompetition {
                competition_id: 26,
                competition: "SA".to_string(),
                is_active: false,
                is_complete: false,
                logo: "https://heat1storage.blob.core.windows.net/competition/southAmerica.png"
                    .to_string(),
                logo_dark:
                    "https://heat1storage.blob.core.windows.net/competition/SouthAmericaWhite.png"
                        .to_string(),
                date: Utc.with_ymd_and_hms(2024, 5, 31, 12, 0, 0).unwrap(),
                heat1_leagues: vec![522, 523],
                men_cut_line: Some(3i64),
                women_cut_line: Some(3i64),
            },
            ActiveCompetition {
                competition_id: 26,
                competition: "SA".to_string(),
                is_active: false,
                is_complete: false,
                logo: "https://heat1storage.blob.core.windows.net/competition/southAmerica.png"
                    .to_string(),
                logo_dark:
                "https://heat1storage.blob.core.windows.net/competition/SouthAmericaWhite.png"
                    .to_string(),
                date: Utc.with_ymd_and_hms(2024, 5, 31, 12, 0, 0).unwrap(),
                heat1_leagues: vec![522, 523],
                men_cut_line: Some(3i64),
                women_cut_line: Some(3i64),
            },
        ];

        Ok(response)
    }

    pub async fn fetch_new_competitor(
        name: String,
    ) -> Result<Vec<NewCompetitionCompetitor>, Error> {
        CompetitorRepository::fetch_competitor(name).await
    }

    pub async fn insert_competition_competitor(
        competiton_id: i64,
        competitor_id: i64,
    ) -> Result<(), Error> {
        CompetitorRepository::create_competition_competitor(competiton_id, competitor_id).await
    }
}
