use crate::data::models::workout::Workout;
use crate::handlers::league::response_models::{
    CompetitionLeaderboardResponse, LeaderboardPicks, LeaderboardScores,
    WorkoutPredictionCountResponse, WorkoutPredictionResponse,
};
use crate::{
    data::{data_client::DataClient, models::tournament::Tournament},
    handlers::league::{
        request_models::{JoinLeague, UserLeaguesRequest},
        response_models::{
            LeaderboardMetadataData, LeaderboardScoreData, LeaderboardShotCallerScoreData,
            LeaderboardTop10ScoreData, LeaderboardTournamentUserData, LeagueAthletesResponse,
            OpenLeagueResponse, UserLeagueTournamentCompetitionStatus,
            UserLeaguesPicksDataResponse, UserLeaguesResponse,
        },
    },
};

use crate::data::models::competition::Competition;
use crate::data::models::competitor::Competitor;
use crate::data::models::score::Score;
use crate::data::models::workout_stage_movement::WorkoutStageMovement;
use crate::data::models::workout_stages::WorkoutStages;
use sqlx::{Error, Row};
use std::collections::HashMap;
use std::hash::Hash;

pub struct LeagueRepository;

#[derive(sqlx::Type)]
#[sqlx(transparent, no_pg_array)]
struct Picks(Vec<(i64, i64, i64)>);

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

    pub async fn fetch_workout_prediction_count(
        competition_id: i64,
        ordinal: i64,
    ) -> Result<HashMap<i64, i64>, Error> {
        let pool = DataClient::connect().await?;

        let res = sqlx::query(
            "
            SELECT
                gender_id,
                COUNT(*) as count
            FROM 
                tournament_user_picks
            JOIN
                tournament_users
                ON tournament_users.id = tournament_user_picks.tournament_user_id
            JOIN
                tournament
                ON tournament.id = tournament_users.tournament_id
            JOIN
                competitor
                ON competitor.id = tournament_user_picks.competitor_id
            WHERE
                tournament.competition_id = $1
                AND tournament_user_picks.rank = $2
                AND tournament.tournament_type_id = 2
            GROUP BY gender_id
            ",
        )
        .bind(competition_id)
        .bind(ordinal)
        .map(
            |row: sqlx::postgres::PgRow| WorkoutPredictionCountResponse {
                gender_id: row.get("gender_id"),
                count: row.get("count"),
            },
        )
        .fetch_all(&pool)
        .await?;

        let result_map = res.into_iter().map(|s| (s.gender_id, s.count)).collect();

        return Ok(result_map);
    }

    pub async fn fetch_workout_picks(
        competition_id: i64,
        ordinal: i64,
        men_picks: i64,
        women_picks: i64,
    ) -> Result<Vec<WorkoutPredictionResponse>, Error> {
        let pool = DataClient::connect().await?;

        let res = sqlx::query(
            "
            SELECT
                CONCAT(competitor.first_name, ' ', competitor.last_name) as competitor,
                COUNT(*) as picks,
                competitor.gender_id
            FROM 
                tournament_user_picks
            JOIN
                tournament_users
                ON tournament_users.id = tournament_user_picks.tournament_user_id
            JOIN
                tournament
                ON tournament.id = tournament_users.tournament_id
            JOIN
                competitor
                ON competitor.id = tournament_user_picks.competitor_id
            WHERE
                tournament.competition_id = $1
                AND tournament_user_picks.rank = $2
                AND tournament.tournament_type_id = 2
            GROUP BY
                gender_id,
                competitor.first_name,
                competitor.last_name
            ",
        )
        .bind(competition_id)
        .bind(ordinal)
        .map(|row: sqlx::postgres::PgRow| {
            let picks = row.get("picks");
            let gender_id = row.get::<i64, _>("gender_id");
            let denominator = if gender_id == 1 {
                men_picks
            } else {
                women_picks
            };
            WorkoutPredictionResponse {
                competitor: row.get("competitor"),
                picks,
                gender_id,
                percentile: (picks as f64) / (denominator as f64) * 100.0,
            }
        })
        .fetch_all(&pool)
        .await?;

        return Ok(res);
    }

    pub async fn fetch_competition_tournament_status(
        user_tournament_id: i64,
    ) -> Result<UserLeagueTournamentCompetitionStatus, Error> {
        let pool = DataClient::connect().await?;

        let res = sqlx::query(
            "
            SELECT
                tournament.tournament_type_id,
                competition.locked_events,
                competition.is_active,
                competition.is_complete 
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
        .bind(user_tournament_id)
        .map(
            |row: sqlx::postgres::PgRow| UserLeagueTournamentCompetitionStatus {
                is_active: row.get("is_active"),
                is_complete: row.get("is_complete"),
                tournament_type_id: row.get("tournament_type_id"),
                locked_events: row.get("locked_events"),
            },
        )
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
            ORDER BY
                ordinal
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

    pub async fn fetch_workout_stages(competition_id: i64) -> Result<Vec<WorkoutStages>, Error> {
        let pool = DataClient::connect().await?;

        let res = sqlx::query(
            "
            SELECT
                ws.id,
                ws.workout_id,
                ws.ordinal,
                ws.time_cap,
                ws.stage_type
            FROM 
                workout_stages as ws
            JOIN
                workouts as w
                ON w.id = ws.workout_id
            WHERE
                w.competition_id = $1
            ORDER BY ws.ordinal ASC
            ",
        )
        .bind(competition_id)
        .map(|row: sqlx::postgres::PgRow| WorkoutStages {
            id: row.get("id"),
            workout_id: row.get("workout_id"),
            ordinal: row.get("ordinal"),
            time_cap: row.get("time_cap"),
            stage_type: row.get("stage_type"),
        })
        .fetch_all(&pool)
        .await?;

        Ok(res)
    }

    pub async fn fetch_workout_stage_movements(
        competition_id: i64,
    ) -> Result<Vec<WorkoutStageMovement>, Error> {
        let pool = DataClient::connect().await?;

        let res = sqlx::query(
            "
            SELECT
                wsm.id,
                wsm.workout_stage_id,
                wsm.ordinal,
                wsm.name
            FROM 
                workout_stage_movement as wsm
            JOIN
                workout_stages as ws
                ON ws.id = wsm.workout_stage_id
            JOIN
                workouts as w
                ON w.id = ws.workout_id
            WHERE
                w.competition_id = $1
            ",
        )
        .bind(competition_id)
        .map(|row: sqlx::postgres::PgRow| WorkoutStageMovement {
            id: row.get("id"),
            workout_stage_id: row.get("workout_stage_id"),
            ordinal: row.get("ordinal"),
            name: row.get("name"),
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

    pub async fn fetch_competition_leaderboard(
        competition_id: i64,
        gender_id: i64,
    ) -> Result<HashMap<i64, CompetitionLeaderboardResponse>, Error> {
        let pool = DataClient::connect().await?;

        let res = sqlx::query(
            "
            SELECT
                competitor.id as competitor_id,
                competitor.gender_id,
                competitor.first_name,
                competitor.last_name,
                COALESCE(points, 0) as points,
                ordinal_finishes,
                COALESCE(placement, 0) as placement
            FROM 
                competition_competitor
            JOIN
                competitor
                ON competitor.id = competition_competitor.competitor_id
            LEFT JOIN
                competition_leaderboard
                ON competitor.id = competition_leaderboard.competitor_id
                AND competition_leaderboard.competition_id = $1
            WHERE
                competition_competitor.competition_id = $1
                AND competitor.gender_id = $2
            ",
        )
        .bind(competition_id)
        .bind(gender_id)
        .map(
            |row: sqlx::postgres::PgRow| CompetitionLeaderboardResponse {
                competitor_id: row.get("competitor_id"),
                competition_id,
                gender_id: row.get("gender_id"),
                first_name: row.get("first_name"),
                last_name: row.get("last_name"),
                points: row.get("points"),
                finishes: row.try_get("ordinal_finishes").unwrap_or(vec![]),
                placement: row.get("placement"),
            },
        )
        .fetch_all(&pool)
        .await?;

        let result_map = res.into_iter().map(|s| (s.competitor_id, s)).collect();

        return Ok(result_map);
    }

    pub async fn fetch_matchup_users(
        tournament_id: i64,
        user_id: i64,
        competitor_id: i64,
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
                AND (tu.id = $2 OR tu.id = $3)
            GROUP BY
                tu.id,
                tu.display_name,
                au.profile_url,
                au.username
            ",
        )
        .bind(tournament_id)
        .bind(user_id)
        .bind(competitor_id)
        .map(|row: sqlx::postgres::PgRow| {
            let picks = row.try_get::<Picks, _>("picks").unwrap_or(Picks(vec![])).0;
            LeaderboardTournamentUserData {
                tournament_user_id: row.get::<i64, _>("tournament_user_id") as u64,
                display_name: "".to_string(),
                avatar: "".to_string(),
                men_competitor_ids: picks
                    .iter()
                    .filter(|p| p.2 == 1)
                    .map(|p| LeaderboardPicks {
                        competitor_id: p.0,
                        rank: p.1,
                    })
                    .collect::<Vec<LeaderboardPicks>>(),
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

            LeaderboardTournamentUserData {
                tournament_user_id: row.get::<i64, _>("tournament_user_id") as u64,
                display_name: if display_name.is_some() {
                    display_name.unwrap()
                } else {
                    row.get("username")
                },
                avatar: row.get("profile_url"),
                men_competitor_ids: picks
                    .iter()
                    .filter(|p| p.2 == 1)
                    .map(|p| LeaderboardPicks {
                        competitor_id: p.0,
                        rank: p.1,
                    })
                    .collect::<Vec<LeaderboardPicks>>(),
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
        .map(|row: sqlx::postgres::PgRow| LeagueAthletesResponse {
            competitor_id: row.get::<i64, _>("competitor_id") as u64,
            gender_id: row.get::<i64, _>("gender_id") as u64,
            first_name: row.get("first_name"),
            last_name: row.get("last_name"),
            ww_rank: row.get::<Option<i64>, _>("ww_rank"),
            adp: row.get::<f64, _>("adp"),
            is_locked: row.get("is_active") || row.get("is_complete"),
            is_withdrawn: row.get("is_withdrawn"),
            is_cut: row.get("is_cut"),
            is_suspended: row.get("is_suspended"),
        })
        .fetch_all(&pool)
        .await?;

        return Ok(res);
    }

    pub async fn fetch_scores(competition_id: i64, ordinal: i64) -> Result<Vec<Score>, Error> {
        let pool = DataClient::connect().await?;

        let res = sqlx::query(
            "
            SELECT
                score.id,
                score.points,
                score.competitor_id
            FROM 
                score
            WHERE
                competition_id = $1
                AND ordinal = $2
            ",
        )
        .bind(competition_id)
        .bind(ordinal)
        .map(|row: sqlx::postgres::PgRow| Score {
            id: row.get::<i64, _>("id") as u64,
            points: row.get("points"),
            competitor_id: row.get::<i64, _>("competitor_id") as u64,
            competition_id: competition_id as u64,
            ordinal: ordinal as u64,
            rank: 0,
            is_scaled: false,
            crossfit_id: "".to_string(),
            breakdown: "".to_string(),
            heat: "".to_string(),
            judge: "".to_string(),
            lane: "".to_string(),
            mobile_score_display: "".to_string(),
            score_display: "".to_string(),
            time: "".to_string(),
            is_valid: false,
            video: "".to_string(),
            year: 0,
            inserted_at: "".to_string(),
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
                AND competition.id >= 13
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
                tournament_user_picks.id,
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
        .bind(*tournament_user_id)
        .map(|row: sqlx::postgres::PgRow| UserLeaguesPicksDataResponse {
            id: row.get("id"),
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
                tournament_id = $1
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

    pub async fn insert_score(
        competition_id: i64,
        competitor_id: i64,
        ordinal: i64,
        points: f64,
    ) -> Result<(), Error> {
        let pool = DataClient::connect().await?;

        sqlx::query(
            "
            INSERT INTO score (competition_id, competitor_id, ordinal, rank, points) 
            VALUES ($1, $2, $3, 0, $4)
            ",
        )
        .bind(competition_id)
        .bind(competitor_id)
        .bind(ordinal)
        .bind(points)
        .execute(&pool)
        .await?;

        return Ok(());
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

    pub async fn refresh_competition_leaderboard() -> Result<(), Error> {
        let pool = DataClient::connect().await?;

        sqlx::query("REFRESH MATERIALIZED VIEW competition_leaderboard")
            .execute(&pool)
            .await?;

        return Ok(());
    }

    pub async fn delete_user_league_pick(tournament_user_pick_id: i64) -> Result<(), Error> {
        let pool = DataClient::connect().await?;

        let _ = sqlx::query(
            "
            DELETE FROM tournament_user_picks
            WHERE id = $1
            ",
        )
        .bind(tournament_user_pick_id)
        .execute(&pool)
        .await?;

        return Ok(());
    }

    pub async fn insert_user_league_pick(
        tournament_user_id: i64,
        competitor_id: i64,
        rank: i64,
    ) -> Result<(), Error> {
        let pool = DataClient::connect().await?;

        sqlx::query(
            "
            INSERT INTO tournament_user_picks (tournament_user_id, competitor_id, rank, last_updated) 
            VALUES ($1, $2, $3, $4)
            ",
        )
        .bind(tournament_user_id)
        .bind(competitor_id)
        .bind(rank)
         .bind(format!("{}", chrono::Utc::now()))
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

    pub async fn fetch_competition_competitor_ids(
        competition_id: i64,
        gender_id: i64,
    ) -> Result<Vec<i64>, Error> {
        let pool = DataClient::connect().await?;

        let res = sqlx::query(
            "
            SELECT
                competitor.id
            FROM 
                competition_competitor
            JOIN
                competitor 
                ON competitor.id = competition_competitor.competitor_id
            WHERE
                competition_competitor.competition_id = $1
                AND competitor.gender_id = $2
            ",
        )
        .bind(competition_id)
        .bind(gender_id)
        .map(|row: sqlx::postgres::PgRow| row.get("id"))
        .fetch_all(&pool)
        .await?;

        return Ok(res);
    }

    pub async fn fetch_competitor_pick_count(
        competition_id: i64,
        gender_id: i64,
    ) -> Result<HashMap<i64, Vec<i64>>, Error> {
        let pool = DataClient::connect().await?;

        let res = sqlx::query(
            "
            SELECT
                tournament_user_picks.competitor_id,
                tournament_user_picks.rank
            FROM 
                tournament_user_picks
            JOIN
                tournament_users
                ON tournament_users.id = tournament_user_picks.tournament_user_id
            JOIN
                tournament 
                ON tournament.id = tournament_users.tournament_id AND tournament.competition_id = $1
            JOIN
                competitor
                ON competitor.id = tournament_user_picks.competitor_id
            WHERE
                competitor.gender_id = $2
                AND tournament.tournament_type_id = 1
            ",
        )
        .bind(competition_id)
        .bind(gender_id)
        .map(|row: sqlx::postgres::PgRow| {
            (
                row.get::<i64, _>("competitor_id"),
                row.get::<i64, _>("rank"),
            )
        })
        .fetch_all(&pool)
        .await?;

        let mut result: HashMap<i64, Vec<i64>> = HashMap::new();

        for (competitor_id, rank) in res {
            result.entry(competitor_id).or_insert(vec![]).push(rank);
        }

        return Ok(result);
    }

    pub async fn fetch_tournament_entries(
        competition_id: i64,
        gender_id: i64,
    ) -> Result<i64, Error> {
        let pool = DataClient::connect().await?;

        let res = sqlx::query(
            "
            SELECT
                COUNT(DISTINCT tournament_user_id) as count
            FROM 
                tournament_user_picks
            JOIN
                tournament_users
                ON tournament_users.id = tournament_user_picks.tournament_user_id
            JOIN
                tournament
                ON tournament.id = tournament_users.tournament_id AND tournament.competition_id = $1
            JOIN
                competitor
                ON competitor.id = tournament_user_picks.competitor_id
            WHERE
                competitor.gender_id = $2
                AND tournament.tournament_type_id = 1
            ",
        )
        .bind(competition_id)
        .bind(gender_id)
        .map(|row: sqlx::postgres::PgRow| row.get("count"))
        .fetch_one(&pool)
        .await?;

        return Ok(res);
    }

    pub async fn update_competitor_adp(competitor_id: i64, adp: f64) -> Result<(), Error> {
        let pool = DataClient::connect().await?;

        let res = sqlx::query(
            "
            UPDATE elite_competitor
            SET adp = $2
            WHERE competitor_id = $1
            ",
        )
        .bind(competitor_id)
        .bind(adp)
        .execute(&pool)
        .await?;

        return Ok(());
    }

    pub async fn update_score(id: i64, points: f64) -> Result<(), Error> {
        let pool = DataClient::connect().await?;

        let _res = sqlx::query(
            "
            UPDATE score
            SET points = $2
            WHERE id = $1
            ",
        )
        .bind(id)
        .bind(points)
        .execute(&pool)
        .await?;

        return Ok(());
    }

    pub async fn update_event(
        competition_id: i64,
        is_active: bool,
        ordinal: i64,
    ) -> Result<(), Error> {
        let pool = DataClient::connect().await?;

        let _res = sqlx::query(
            "
            UPDATE competition
            SET is_active = $2, locked_events = $3
            WHERE id = $1
            ",
        )
        .bind(competition_id)
        .bind(is_active)
        .bind(ordinal)
        .execute(&pool)
        .await?;

        return Ok(());
    }

    pub async fn update_workout(
        competition_id: i64,
        is_active: bool,
        ordinal: i64,
    ) -> Result<(), Error> {
        let pool = DataClient::connect().await?;

        let _res = sqlx::query(
            "
            UPDATE workouts
            SET is_active = $2
            WHERE competition_id = $1 AND ordinal = $3
            ",
        )
        .bind(competition_id)
        .bind(is_active)
        .bind(ordinal)
        .execute(&pool)
        .await?;

        return Ok(());
    }
}
