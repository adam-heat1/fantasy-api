use crate::data::models::workout::Workout;
use crate::handlers::league::response_models::{LeaderboardPicks, LeaderboardScores};
use crate::{
    data::{data_client::DataClient, models::tournament::Tournament},
    handlers::league::{
        request_models::{JoinLeague, UserLeaguesRequest},
        response_models::{
            LeaderboardMetadataData, LeaderboardScoreData, LeaderboardShotCallerScoreData,
            LeaderboardTop10ScoreData, LeaderboardTournamentUserData, LeagueAthletesResponse,
            OpenLeagueResponse, UserLeaguesPicksDataResponse, UserLeaguesResponse,
        },
    },
};

use sqlx::{Error, Row};
use std::collections::HashMap;

pub struct LeagueRepository;

#[derive(sqlx::Type)]
#[sqlx(transparent, no_pg_array)]
struct Picks(Vec<(i64, i64, i64)>);

#[derive(sqlx::Type)]
#[sqlx(transparent, no_pg_array)]
struct Scores(Vec<(i64, f64, i64)>);
impl LeagueRepository {
    pub async fn fetch_competition(tournament_id: i64) -> Result<LeaderboardMetadataData, Error> {
        let pool = DataClient::connect().await?;

        let res = sqlx::query(
            "
            SELECT
                competition.id as competition_id,
                competition.name as competition_name,
                competition.logo as competition_logo,
                competition.locked_events,
                tournament.name as tournament_name,
                tournament.tournament_type_id 
            FROM 
                competition
            JOIN
                tournament
                ON tournament.competition_id = competition.id
            WHERE
                tournament.id = $1
            ",
        )
        .bind(tournament_id)
        .map(|row: sqlx::postgres::PgRow| LeaderboardMetadataData {
            competition_id: row.get::<i64, _>("competition_id") as u64,
            competition_name: row.get("competition_name"),
            competition_logo: row.get("competition_logo"),
            locked_events: row.get::<i64, _>("locked_events") as u64,
            tournament_name: row.get("tournament_name"),
            tournament_type_id: row.get::<i64, _>("tournament_type_id") as u64,
        })
        .fetch_one(&pool)
        .await?;

        return Ok(res);
    }

    pub async fn fetch_workouts(competition_id: i64) -> Result<Vec<Workout>, Error> {
        let pool = DataClient::connect().await?;

        let res = sqlx::query(
            "
            SELECT
                id,
                name,
                ordinal,
                is_active,
                is_complete,
                start_time,
                location,
                description
            FROM 
                workouts
            WHERE
                competition_id = $1
            ",
        )
        .bind(competition_id)
        .map(|row: sqlx::postgres::PgRow| Workout {
            id: row.get("id"),
            name: row.get("name"),
            ordinal: row.get("ordinal"),
            start_time: row.get("start_time"),
            description: row.get("description"),
            location: row.get("location"),
            is_active: row.get("is_active"),
            is_complete: row.get("is_complete"),
        })
        .fetch_all(&pool)
        .await?;

        Ok(res)
    }

    pub async fn fetch_user_tournament_competition_id(
        tournament_user_id: &i64,
    ) -> Result<i64, Error> {
        let pool = DataClient::connect().await?;

        let res = sqlx::query(
            "
            SELECT
                competition.id
            FROM 
                competition
            JOIN
                tournament
                ON tournament.competition_id = competition.id
            JOIN
                tournament_users
                ON tournament_users.tournament_id = tournament.id
            WHERE
                tournament_users.id = $1
            ",
        )
        .bind(*tournament_user_id)
        .map(|row: sqlx::postgres::PgRow| row.get::<i64, _>("id"))
        .fetch_one(&pool)
        .await?;

        return Ok(res);
    }

    pub async fn fetch_scores(
        competition_id: u64,
        gender_id: i64,
    ) -> Result<HashMap<u64, LeaderboardScoreData>, Error> {
        let pool = DataClient::connect().await?;

        let res = sqlx::query(
            "
            SELECT
                score.points,
                score.ordinal,
                score.rank,
                score.competitor_id
            FROM 
                score
            JOIN
                competitor
                ON score.competitor_id = competitor.id
            WHERE
                score.competition_id = $1
                AND competitor.gender_id = $2
            ",
        )
        .bind(competition_id as i64)
        .bind(gender_id)
        .map(|row: sqlx::postgres::PgRow| {
            let points = row.get::<Option<f64>, _>("points");
            LeaderboardScoreData {
                points,
                ordinal: row.get::<i64, _>("ordinal") as u64,
                rank: row.get::<i64, _>("rank") as u64,
                competitor_id: row.get::<i64, _>("competitor_id") as u64,
                gender_id: gender_id as u64,
            }
        })
        .fetch_all(&pool)
        .await?;

        let result_map = res.into_iter().map(|s| (s.competitor_id, s)).collect();

        return Ok(result_map);
    }

    pub async fn fetch_shot_caller_scores(
        competition_id: u64,
    ) -> Result<Vec<LeaderboardShotCallerScoreData>, Error> {
        let pool = DataClient::connect().await?;

        let res = sqlx::query(
            "
            SELECT
                s.competitor_id,
                ARRAY_AGG((s.ordinal, s.points, c.gender_id)) as scores
            FROM 
                score as s
            JOIN
                competitor as c
                ON s.competitor_id = c.id
            WHERE
                s.competition_id = $1
            GROUP BY
                s.competitor_id
            ",
        )
        .bind(competition_id as i64)
        .map(|row: sqlx::postgres::PgRow| {
            let scores = row
                .try_get::<Scores, _>("scores")
                .unwrap_or(Scores(vec![]))
                .0;
            let men_scores = scores
                .iter()
                .filter(|p| p.2 == 1)
                .map(|p| LeaderboardScores {
                    ordinal: p.0,
                    points: p.1,
                })
                .collect::<Vec<LeaderboardScores>>();
            LeaderboardShotCallerScoreData {
                men_competitors: men_scores,
                women_competitors: scores
                    .iter()
                    .filter(|p| p.2 == 1)
                    .map(|p| LeaderboardScores {
                        ordinal: p.0,
                        points: p.1,
                    })
                    .collect::<Vec<LeaderboardScores>>(),
                competitor_id: row.get::<i64, _>("competitor_id") as u64,
            }
        })
        .fetch_all(&pool)
        .await?;

        return Ok(res);
    }

    pub async fn fetch_top_10_scores(
        competition_id: u64,
        gender_id: i64,
    ) -> Result<Vec<LeaderboardTop10ScoreData>, Error> {
        let pool = DataClient::connect().await?;

        let res = sqlx::query(
            "
            SELECT
                SUM(score.points) as points,
                score.competitor_id
            FROM 
                score
            JOIN
                competitor
                ON score.competitor_id = competitor.id
            WHERE
                score.competition_id = $1
                AND competitor.gender_id = $2
            GROUP BY
                score.competitor_id
            ",
        )
        .bind(competition_id as i64)
        .bind(gender_id)
        .map(|row: sqlx::postgres::PgRow| {
            let points = row.get::<Option<f64>, _>("points");
            LeaderboardTop10ScoreData {
                points,
                rank: 0,
                competitor_id: row.get::<i64, _>("competitor_id") as u64,
            }
        })
        .fetch_all(&pool)
        .await?;

        return Ok(res);
    }

    pub async fn fetch_tournament_users(
        tournament_id: i64,
    ) -> Result<Vec<LeaderboardTournamentUserData>, Error> {
        let pool = DataClient::connect().await?;

        let res = sqlx::query(
            "
            SELECT
                tu.id as tournament_user_id,
                tu.display_name,
                au.profile_url,
                au.username,
                ARRAY_AGG((tup.competitor_id, tup.rank, c.gender_id)) as picks
            FROM 
                tournament_users as tu
            JOIN
                app_user as au
                ON tu.user_id = au.id
            LEFT JOIN
                tournament_user_picks as tup
                ON tup.tournament_user_id = tu.id
            LEFT JOIN
                competitor as c
                ON c.id = tup.competitor_id
            WHERE
                tu.tournament_id = $1
            GROUP BY
                tu.id,
                tu.display_name,
                au.profile_url,
                au.username
            ",
        )
        .bind(tournament_id)
        .map(|row: sqlx::postgres::PgRow| {
            let display_name = row.get::<Option<String>, _>("display_name");
            let picks = row.try_get::<Picks, _>("picks").unwrap_or(Picks(vec![])).0;
            let men_picks = picks
                .iter()
                .filter(|p| p.2 == 1)
                .map(|p| LeaderboardPicks {
                    competitor_id: p.0,
                    rank: p.1,
                })
                .collect::<Vec<LeaderboardPicks>>();
            LeaderboardTournamentUserData {
                tournament_user_id: row.get::<i64, _>("tournament_user_id") as u64,
                display_name: if display_name.is_some() {
                    display_name.unwrap()
                } else {
                    row.get("username")
                },
                avatar: row.get("profile_url"),
                men_competitor_ids: men_picks,
                women_competitor_ids: picks
                    .iter()
                    .filter(|p| p.2 == 2)
                    .map(|p| LeaderboardPicks {
                        competitor_id: p.0,
                        rank: p.1,
                    })
                    .collect::<Vec<LeaderboardPicks>>(),
            }
        })
        .fetch_all(&pool)
        .await?;

        return Ok(res);
    }

    pub async fn fetch_league_athletes(
        competition_id: u64,
    ) -> Result<Vec<LeagueAthletesResponse>, Error> {
        let pool = DataClient::connect().await?;

        let res = sqlx::query(
            "
            SELECT
                competition_competitor.competitor_id,
                competition_competitor.is_withdrawn,
                competition_competitor.is_cut,
                competition_competitor.is_suspended,
                competitor.first_name,
                competitor.last_name,
                competitor.gender_id,
                competition.is_active,
                competition.is_complete,
                elite_competitor.ww_rank,
                elite_competitor.adp
            FROM 
                competition_competitor
            JOIN
                competitor 
                ON competitor.id = competition_competitor.competitor_id
            JOIN
                competition 
                ON competition.id = competition_competitor.competition_id
            LEFT JOIN
                elite_competitor
                ON elite_competitor.competitor_id = competitor.id
            WHERE
                competition.id = $1
            ",
        )
        .bind(competition_id as i64)
        .map(|row: sqlx::postgres::PgRow| {
            let ww_rank = row.get::<Option<i64>, _>("ww_rank");
            // let adp = row.get::<f64, _>("adp");
            LeagueAthletesResponse {
                competitor_id: row.get::<i64, _>("competitor_id") as u64,
                gender_id: row.get::<i64, _>("gender_id") as u64,
                first_name: row.get("first_name"),
                last_name: row.get("last_name"),
                ww_rank: if ww_rank.is_some() {
                    Some(ww_rank.unwrap() as u64)
                } else {
                    Some(0)
                },
                adp: Some(0.1),
                is_locked: row.get("is_active") || row.get("is_complete"),
                is_withdrawn: row.get("is_withdrawn"),
                is_cut: row.get("is_cut"),
                is_suspended: row.get("is_suspended"),
            }
        })
        .fetch_all(&pool)
        .await?;

        return Ok(res);
    }

    pub async fn fetch_user_leagues(
        user_league: &UserLeaguesRequest,
    ) -> Result<Vec<UserLeaguesResponse>, Error> {
        let pool = DataClient::connect().await?;

        let res = sqlx::query(
            "
            SELECT
                tournament_users.id as tournament_users_id,
                tournament_users.display_name,
                competition.name as competition_name,
                competition.id as competition_id,
                tournament.name as tournament_name,
                tournament.id as tournament_id,
                competition.logo,
                competition.locked_events,
                competition.is_active,
                competition.is_complete,
                tournament.tournament_type_id
            FROM 
                tournament_users
            JOIN
                tournament
                ON tournament.id = tournament_users.tournament_id
            JOIN 
                competition
                ON competition.id = tournament.competition_id 
            WHERE
                tournament_users.user_id = $1
            ",
        )
        .bind(user_league.user_id)
        .map(|row: sqlx::postgres::PgRow| UserLeaguesResponse {
            tournament_user_id: row.get::<i64, _>("tournament_users_id") as u64,
            display_name: row.get::<Option<String>, _>("display_name"),
            competition: row.get("competition_name"),
            competition_id: row.get::<i64, _>("competition_id") as u64,
            tournament: row.get("tournament_name"),
            tournament_id: row.get::<i64, _>("tournament_id") as u64,
            logo: row.get("logo"),
            is_active: row.get("is_active"),
            is_complete: row.get("is_complete"),
            locked_events: row.get::<i64, _>("locked_events") as u64,
            tournament_type_id: row.get::<i64, _>("tournament_type_id") as u64,
        })
        .fetch_all(&pool)
        .await?;

        return Ok(res);
    }

    pub async fn fetch_user_league_picks(
        tournament_user_id: &i64,
    ) -> Result<Vec<UserLeaguesPicksDataResponse>, Error> {
        let pool = DataClient::connect().await?;

        let res = sqlx::query(
            "
            SELECT
                tournament_user_picks.competitor_id,
                tournament_user_picks.rank,
                competitor.gender_id
            FROM 
                tournament_user_picks
            JOIN
                competitor
                ON competitor.id = tournament_user_picks.competitor_id
            WHERE
                tournament_user_picks.tournament_user_id = $1
            ",
        )
        .bind(*tournament_user_id as i64)
        .map(|row: sqlx::postgres::PgRow| UserLeaguesPicksDataResponse {
            competitor_id: row.get::<i64, _>("competitor_id") as u64,
            rank: row.get::<i64, _>("rank") as u64,
            gender_id: row.get::<i64, _>("gender_id") as u64,
        })
        .fetch_all(&pool)
        .await?;

        return Ok(res);
    }

    pub async fn fetch_is_user_in_league(join_league: &JoinLeague) -> Result<bool, Error> {
        let pool = DataClient::connect().await?;

        let res = sqlx::query(
            "
            SELECT
                id
            FROM
                tournament_users
            WHERE
                id = $1
                AND user_id = $2
            ",
        )
        .bind(join_league.tournament_id)
        .bind(join_league.user_id)
        .fetch_all(&pool)
        .await?;

        let is_user_in_league = res.len() > 0;

        return Ok(is_user_in_league);
    }

    pub async fn fetch_open_leagues(
        competition_id: &u64,
        user_id: &u64,
    ) -> Result<Vec<OpenLeagueResponse>, Error> {
        let pool = DataClient::connect().await?;

        let res = sqlx::query(
            "
            SELECT
                tournament.id,
                competition_id,
                name,
                tournament_type_id,
                is_private,
                passcode,
                (SELECT COUNT(*) FROM tournament_users WHERE tournament_users.tournament_id = tournament.id) as Entries
            FROM
                tournament
            LEFT JOIN tournament_users
                ON tournament_users.tournament_id = tournament.id
                AND tournament_users.user_id = $2
            WHERE
                tournament.competition_id = $1
                AND tournament_users IS NULL
            ",
        )
            .bind(*competition_id as i64)
            .bind(*user_id as i64)
            .map(|row: sqlx::postgres::PgRow| OpenLeagueResponse {
                id: row.get::<i64, _>("id") as u64,
                competition_id: row.get::<i64, _>("competition_id") as u64,
                name: row.get("name"),
                tournament_type_id: row.get::<i64, _>("tournament_type_id") as u64,
                is_private: row.get("is_private"),
                passcode: row.get("passcode"),
                entries: row.get::<i64, _>("entries") as u64,
            })
            .fetch_all(&pool)
            .await?;

        return Ok(res);
    }
    pub async fn insert_tournament_user(tournament_id: i64, user_id: i64) -> Result<(), Error> {
        let pool = DataClient::connect().await?;

        sqlx::query(
            "
            INSERT INTO tournament_users (tournament_id, user_id) 
            VALUES ($1, $2)
            ",
        )
        .bind(tournament_id)
        .bind(user_id)
        .execute(&pool)
        .await?;

        return Ok(());
    }

    pub async fn insert_tournament(tournament: Tournament) -> Result<u64, Error> {
        let pool = DataClient::connect().await?;

        let res = sqlx::query(
            "
            INSERT INTO
                tournament
            (competition_id, name, tournament_type_id, is_private, passcode, commissioner_id)
            VALUES 
                ($1, $2, $3, $4, $5, $6)
            RETURNING
                id
            ",
        )
        .bind(tournament.competition_id as i64)
        .bind(tournament.name)
        .bind(tournament.tournament_type_id as i64)
        .bind(tournament.is_private)
        .bind(tournament.passcode)
        .bind(tournament.commissioner_id as i64)
        .fetch_one(&pool)
        .await?;

        let id = res.get::<i64, _>("id") as u64;

        return Ok(id);
    }

    //     pub async fn fetch_leagues_by_user_id(user_id: u64) -> Result<Vec<TournamentUsers>, Error> {
    //         let pool = DataClient::connect().await?;

    //         let res = sqlx::query(
    //             "
    //             SELECT
    //                 tournament_users.id as tournament_users_id,
    //                 tournament_users.display_name,
    //                 tournament.id as tournament_id,
    //                 tournament.name as tournament_name,
    //                 tournament.logo,
    //                 tournament.tournament_type_id,
    //                 competition.id as competition_id,
    //                 competition.name as competition_name,
    //                 competition.is_active,
    //                 competition.is_complete,
    //                 competition.logo,
    //                 competition.region_id,
    //                 competition.sort_order
    //             FROM
    //                 tournament_users
    //             JOIN
    //                 tournament on tournament_users.tournament_id = tournament.id
    //             JOIN
    //                 competition on tournament.competition_id = competition.id
    //             WHERE
    //                 user_id = $1
    //             ",
    //         )
    //         .bind(user_id as i64)
    //         .map(|row: PgRow| {
    //             let competition = Competition {
    //                 id: row.get::<i64, _>("competition_id") as u64,
    //                 name: row.get("competition_name"),
    //                 is_active: row.get("is_active"),
    //                 is_complete: row.get("is_complete"),
    //                 logo: row.get("logo"),
    //                 region_id: row.get::<i64, _>("region_id") as u64,
    //                 sort_order: row.get::<i64, _>("sort_order") as u64,
    //                 region: None,
    //             };

    //             let tournament = Tournament {
    //                 id: row.get::<i64, _>("tournament_id") as u64,
    //                 name: row.get("competition_name"),
    //                 logo: row.get("logo"),
    //                 tournament_type_id: row.get::<i64, _>("tournament_type_id") as u64,
    //                 competition_id: row.get::<i64, _>("competition_id") as u64,
    //                 locked_events: 0,
    //                 tournament_type: None,
    //                 competition: Some(competition),
    //             };

    //             return TournamentUsers {
    //                 id: row.get::<i64, _>("tournament_users_id") as u64,
    //                 display_name: row.get("display_name"),
    //                 tournament_id: row.get::<i64, _>("tournament_id") as u64,
    //                 user_id,
    //                 tournament,
    //                 user: None,
    //             };
    //         })
    //         .fetch_all(&pool)
    //         .await?;

    //         return Ok(res);
    //     }
}
