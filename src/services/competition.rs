use crate::handlers::competition::response_models::{ActiveCompetition, NewCompetitionCompetitor};
use crate::repositories::competitor::CompetitorRepository;
use chrono::{TimeZone, Utc};
use sqlx::Error;

pub struct CompetitionService;

impl CompetitionService {
    pub async fn fetch_active_beta_competitions() -> Result<Vec<ActiveCompetition>, Error> {
        let d = Utc.with_ymd_and_hms(2024, 5, 16, 12, 0, 0).unwrap();
        println!("{}", d);
        let response: Vec<ActiveCompetition> = vec![
            ActiveCompetition {
                competition_id: 28,
                competition: "CrossFit Games".to_string(),
                is_active: false,
                is_complete: false,
                logo: "https://heat1storage.blob.core.windows.net/competition/crossfitGames2.jpg"
                    .to_string(),
                logo_dark:
                    "https://heat1storage.blob.core.windows.net/competition/crossfitGames2.jpg"
                        .to_string(),
                date: Utc.with_ymd_and_hms(2024, 8, 01, 12, 0, 0).unwrap(),
                heat1_leagues: vec![759, 765],
                is_top10_enabled: true,
                is_shotcaller_enabled: true,
                men_cut_line: None,
                women_cut_line: None,
            },
            ActiveCompetition {
                competition_id: 29,
                competition: "TYR WZA SoCal".to_string(),
                is_active: false,
                is_complete: false,
                logo: "https://heat1storage.blob.core.windows.net/competition/wza.png".to_string(),
                logo_dark: "https://heat1storage.blob.core.windows.net/competition/wza.png"
                    .to_string(),
                date: Utc.with_ymd_and_hms(2024, 9, 20, 12, 0, 0).unwrap(),
                heat1_leagues: vec![],
                is_top10_enabled: false,
                is_shotcaller_enabled: true,
                men_cut_line: None,
                women_cut_line: None,
            },
            ActiveCompetition {
                competition_id: 30,
                competition: "Crash Crucible".to_string(),
                is_active: false,
                is_complete: false,
                logo: "https://heat1storage.blob.core.windows.net/competition/crashCrucible.jpg"
                    .to_string(),
                logo_dark:
                    "https://heat1storage.blob.core.windows.net/competition/crashCrucible.jpg"
                        .to_string(),
                date: Utc.with_ymd_and_hms(2024, 10, 20, 12, 0, 0).unwrap(),
                heat1_leagues: vec![],
                is_top10_enabled: true,
                is_shotcaller_enabled: true,
                men_cut_line: None,
                women_cut_line: None,
            },
            ActiveCompetition {
                competition_id: 31,
                competition: "Rogue Invitational".to_string(),
                is_active: false,
                is_complete: false,
                logo:
                    "https://heat1storage.blob.core.windows.net/competition/rogueInvitational.jpg"
                        .to_string(),
                logo_dark:
                    "https://heat1storage.blob.core.windows.net/competition/rogueInvitational.jpg"
                        .to_string(),
                date: Utc.with_ymd_and_hms(2024, 9, 20, 12, 0, 0).unwrap(),
                heat1_leagues: vec![],
                is_top10_enabled: true,
                is_shotcaller_enabled: true,
                men_cut_line: None,
                women_cut_line: None,
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
