use crate::data::models::workout::Workout;
use crate::handlers::league::response_models::{
    CompetitionLeaderboardResponse, LeaderboardEntry, LeaderboardPicks, LeaderboardShotcallerPicks,
    LeaderboardShotcallerTournamentUserData, LeaguePosition, MatchupShotcallerPick, PickCompetitor,
    PickPercentage, PropBet, PropBetOption, UserLeaguesTopPicksDataResponse,
    WorkoutPredictionCountResponse, WorkoutPredictionResponse, WorkoutResponse,
};
use crate::{
    data::{data_client::DataClient, models::tournament::Tournament},
    handlers::league::{
        request_models::{JoinLeague, UserLeaguesRequest},
        response_models::{
            LeaderboardMetadataData, LeaderboardTournamentUserData, LeagueAthletesResponse,
            OpenLeagueResponse, UserLeagueTournamentCompetitionStatus,
            UserLeaguesPicksDataResponse, UserLeaguesResponse,
        },
    },
};

use crate::data::models::score::Score;
use crate::data::models::workout_stage_movement::WorkoutStageMovement;
use crate::data::models::workout_stages::WorkoutStages;
use crate::data::tournament_pick_count::TournamentPickCount;
use sqlx::postgres::PgRow;
use sqlx::{Error, Row};
use std::collections::HashMap;

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
                tournament.tournament_type_id,
                tournament.pick_count
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
            pick_count: row.get("pick_count"),
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
        .map(|row: PgRow| WorkoutPredictionCountResponse {
            gender_id: row.get("gender_id"),
            count: row.get("count"),
        })
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

    pub async fn fetch_top_10_tournaments(
        competition_id: i64,
    ) -> Result<Vec<TournamentPickCount>, Error> {
        let pool = DataClient::connect().await?;

        let res = sqlx::query(
            "
            SELECT
                tournament.id,
                tournament.pick_count
            FROM
                tournament
            WHERE
                tournament.competition_id = $1
                AND tournament.tournament_type_id = 1
            ",
        )
        .bind(competition_id)
        .map(|row: PgRow| TournamentPickCount {
            id: row.get("id"),
            pick_count: row.get("pick_count"),
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

    pub async fn fetch_pick_competitor(
        tournament_user_pick_id: i64,
    ) -> Result<PickCompetitor, Error> {
        let pool = DataClient::connect().await?;

        let res = sqlx::query(
            "
            SELECT
                tup.id,
                tup.competitor_id,
                tup.rank,
                tup.tournament_position_id
            FROM
                tournament_user_picks tup
            WHERE
                tup.id = $1
            ",
        )
        .bind(tournament_user_pick_id)
        .map(|row: PgRow| PickCompetitor {
            competitor_id: row.get("competitor_id"),
            rank: row.get("rank"),
            id: row.get("id"),
            tournament_position_id: row.get("tournament_position_id"),
        })
        .fetch_one(&pool)
        .await?;

        return Ok(res);
    }

    pub async fn fetch_competition_tournament_status_by_pick(
        tournament_user_pick_id: i64,
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
            JOIN
                tournament_user_picks
                ON tournament_user_picks.tournament_user_id = tournament_users.id
            WHERE
                tournament_user_picks.id = $1
            ",
        )
        .bind(tournament_user_pick_id)
        .map(|row: PgRow| UserLeagueTournamentCompetitionStatus {
            is_active: row.get("is_active"),
            is_complete: row.get("is_complete"),
            tournament_type_id: row.get("tournament_type_id"),
            locked_events: row.get("locked_events"),
        })
        .fetch_one(&pool)
        .await?;

        return Ok(res);
    }

    pub async fn fetch_shot_caller_pick_id(
        tournament_user_id: i64,
        workout_id: i64,
        tournament_position_id: i64,
    ) -> Result<Option<i64>, Error> {
        let pool = DataClient::connect().await?;

        let res = sqlx::query(
            "
            SELECT
                tup.id
            FROM
                tournament_user_picks tup
            WHERE
                tup.tournament_user_id = $1
                AND tup.workout_id = $2
                AND tup.tournament_position_id = $3
            ",
        )
        .bind(tournament_user_id)
        .bind(workout_id)
        .bind(tournament_position_id)
        .map(|row: PgRow| row.get("id"))
        .fetch_optional(&pool)
        .await?;

        return Ok(res);
    }

    pub async fn fetch_competitor_gender_id(competitor_id: i64) -> Result<i64, Error> {
        let pool = DataClient::connect().await?;

        let res = sqlx::query(
            "
            SELECT
                c.gender_id
            FROM
                competitor c
            WHERE
                c.id = $1
            ",
        )
        .bind(competitor_id)
        .map(|row: PgRow| row.get("gender_id"))
        .fetch_one(&pool)
        .await?;

        return Ok(res);
    }

    pub async fn fetch_top_pick_id(
        tournament_user_id: i64,
        gender_id: i64,
        tournament_position_id: i64,
    ) -> Result<Option<i64>, Error> {
        let pool = DataClient::connect().await?;

        let res = sqlx::query(
            "
            SELECT
                tup.id
            FROM
                tournament_user_picks tup
            JOIN
                competitor c
                ON c.id = tup.competitor_id
            WHERE
                tup.tournament_user_id = $1
                AND c.gender_id = $2
                AND tup.tournament_position_id = $3
            ",
        )
        .bind(tournament_user_id)
        .bind(gender_id)
        .bind(tournament_position_id)
        .map(|row: PgRow| row.get("id"))
        .fetch_optional(&pool)
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
                description,
                sponsor,
                sponsor_link,
                sponsor_logo,
                sponsor_logo_dark
            FROM
                workouts
            WHERE
                competition_id = $1
            ORDER BY
                ordinal
            ",
        )
        .bind(competition_id)
        .map(|row: PgRow| Workout {
            id: row.get("id"),
            name: row.get("name"),
            ordinal: row.get("ordinal"),
            start_time: row.get("start_time"),
            description: row.get("description"),
            location: row.get("location"),
            is_active: row.get("is_active"),
            is_complete: row.get("is_complete"),
            sponsor: row.get("sponsor"),
            sponsor_link: row.get("sponsor_link"),
            sponsor_logo: row.get("sponsor_logo"),
            sponsor_logo_dark: row.get("sponsor_logo_dark"),
        })
        .fetch_all(&pool)
        .await?;

        Ok(res)
    }

    pub async fn fetch_workouts_by_tournament(
        tournament_id: i64,
    ) -> Result<Vec<WorkoutResponse>, Error> {
        let pool = DataClient::connect().await?;

        let res = sqlx::query(
            "
            SELECT
                w.id,
                w.name,
                w.ordinal,
                w.is_active,
                w.is_complete,
                w.start_time,
                w.location,
                w.description,
                w.sponsor,
                w.sponsor_link,
                w.sponsor_logo,
                w.sponsor_logo_dark
            FROM
                workouts w
            JOIN
                tournament t
                ON t.competition_id = w.competition_id
            WHERE
                t.id = $1
            ",
        )
        .bind(tournament_id)
        .map(|row: PgRow| WorkoutResponse {
            id: row.get("id"),
            name: row.get("name"),
            ordinal: row.get("ordinal"),
            start_time: row.get("start_time"),
            description: row.get("description"),
            location: row.get("location"),
            is_active: row.get("is_active"),
            is_complete: row.get("is_complete"),
            sponsor: row.get("sponsor"),
            sponsor_link: row.get("sponsor_link"),
            sponsor_logo: row.get("sponsor_logo"),
            sponsor_logo_dark: row.get("sponsor_logo_dark"),
            stages: None,
        })
        .fetch_all(&pool)
        .await?;

        Ok(res)
    }

    pub async fn fetch_workout(workout_id: i64) -> Result<Workout, Error> {
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
                description,
                sponsor,
                sponsor_link,
                sponsor_logo,
                sponsor_logo_dark
            FROM
                workouts
            WHERE
                id = $1
            ",
        )
        .bind(workout_id)
        .map(|row: PgRow| Workout {
            id: row.get("id"),
            name: row.get("name"),
            ordinal: row.get("ordinal"),
            start_time: row.get("start_time"),
            description: row.get("description"),
            location: row.get("location"),
            is_active: row.get("is_active"),
            is_complete: row.get("is_complete"),
            sponsor: row.get("sponsor"),
            sponsor_link: row.get("sponsor_link"),
            sponsor_logo: row.get("sponsor_logo"),
            sponsor_logo_dark: row.get("sponsor_logo_dark"),
        })
        .fetch_one(&pool)
        .await?;

        Ok(res)
    }

    pub async fn fetch_workout_by_pick(tournament_user_pick_id: i64) -> Result<Workout, Error> {
        let pool = DataClient::connect().await?;

        let res = sqlx::query(
            "
            SELECT
                workouts.id,
                workouts.name,
                workouts.ordinal,
                workouts.is_active,
                workouts.is_complete,
                workouts.start_time,
                workouts.location,
                workouts.description,
                workouts.sponsor,
                workouts.sponsor_link,
                workouts.sponsor_logo,
                workouts.sponsor_logo_dark
            FROM
                workouts
            JOIN
                tournament_user_picks
                ON tournament_user_picks.workout_id = workouts.id
            WHERE
                tournament_user_picks.id = $1
            ",
        )
        .bind(tournament_user_pick_id)
        .map(|row: PgRow| Workout {
            id: row.get("id"),
            name: row.get("name"),
            ordinal: row.get("ordinal"),
            start_time: row.get("start_time"),
            description: row.get("description"),
            location: row.get("location"),
            is_active: row.get("is_active"),
            is_complete: row.get("is_complete"),
            sponsor: row.get("sponsor"),
            sponsor_link: row.get("sponsor_link"),
            sponsor_logo: row.get("sponsor_logo"),
            sponsor_logo_dark: row.get("sponsor_logo_dark"),
        })
        .fetch_one(&pool)
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

    pub async fn fetch_shotcaller_props_by_competition(
        competition_id: i64,
    ) -> Result<Vec<PropBet>, Error> {
        let pool = DataClient::connect().await?;

        let res = sqlx::query(
            "
            SELECT
                prop_bets.id,
                prop_bets.workout_id,
                prop_bets.name,
                prop_bets.start_time,
                prop_bets.ordinal,
                prop_bets.is_active,
                prop_bets.is_complete,
                prop_bets.description
            FROM
                prop_bets
            JOIN
                workouts ON workouts.id = prop_bets.workout_id
            WHERE
                prop_bets.is_hidden = false
                AND workouts.competition_id = $1
            ORDER BY
                workouts.id,
                prop_bets.ordinal
            ",
        )
        .bind(competition_id)
        .map(|row: PgRow| PropBet {
            id: row.get("id"),
            name: row.get("name"),
            start_time: row.get("start_time"),
            ordinal: row.get("ordinal"),
            is_active: row.get("is_active"),
            is_complete: row.get("is_complete"),
            description: row.get("description"),
            options: vec![],
        })
        .fetch_all(&pool)
        .await?;

        return Ok(res);
    }

    pub async fn fetch_shotcaller_prop_options(
        competition_id: i64,
        tournament_user_id: i64,
    ) -> Result<HashMap<i64, Vec<PropBetOption>>, Error> {
        let pool = DataClient::connect().await?;

        let res = sqlx::query(
            "
            SELECT
                prop_options.id,
                prop_options.prop_bet_id,
                prop_options.name,
                prop_options.image_url,
                prop_options.points,
                CASE WHEN prop_picks.id IS NULL THEN false ELSE true END AS is_picked
            FROM
                prop_options
            JOIN
                prop_bets ON prop_bets.id = prop_options.prop_bet_id
            JOIN
                workouts ON workouts.id = prop_bets.workout_id
            LEFT JOIN
                prop_picks ON prop_picks.prop_option_id = prop_options.id
                    AND prop_picks.tournament_user_id = $2
            WHERE
                prop_bets.is_hidden = false
                AND workouts.competition_id = $1
                AND (prop_picks.is_valid IS NULL OR prop_picks.is_valid = true)
            ORDER BY
                prop_options.points DESC
            ",
        )
        .bind(competition_id)
        .bind(tournament_user_id)
        .map(|row: PgRow| PropBetOption {
            id: row.get("id"),
            prop_bet_id: row.get("prop_bet_id"),
            name: row.get("name"),
            image_url: row.get("image_url"),
            points: row.get("points"),
            is_picked: row.get("is_picked"),
            percentage: 0.0,
        })
        .fetch_all(&pool)
        .await?;

        let mut result: HashMap<i64, Vec<PropBetOption>> = HashMap::new();

        for r in res {
            result.entry(r.prop_bet_id).or_insert(vec![]).push(r);
        }

        return Ok(result);
    }

    pub async fn fetch_top_10_leaderboard(
        tournament_id: i64,
        competition_id: i64,
    ) -> Result<Vec<LeaderboardEntry>, Error> {
        let pool = DataClient::connect().await?;

        let res = sqlx::query(
            "
            SELECT
                tu.id as tournament_user_id,
                au.username,
                au.profile_url,
                SUM(CASE WHEN 10 - ABS(rank - placement) < 0 THEN 0 ELSE 10 - ABS(rank - placement) END) as points,
                SUM(CASE WHEN 10 - ABS(rank - placement) = 10 THEN 1 ELSE 0 END) as exact_picks,
                RANK() OVER (
                    ORDER BY COALESCE(
                        SUM(CASE WHEN 10 - ABS(rank - placement) < 0 THEN 0 ELSE 10 - ABS(rank - placement) END), 0::double precision) DESC,
                        SUM(CASE WHEN 10 - ABS(rank - placement) = 10 THEN 1 ELSE 0 END) DESC
                ) AS ordinal
            FROM tournament_users tu
                JOIN app_user au
                    ON au.id = tu.user_id
                LEFT JOIN tournament_user_picks tup
                    ON tup.tournament_user_id = tu.id
                LEFT JOIN competition_leaderboard s
                    ON s.competitor_id = tup.competitor_id
                    AND s.competition_id = $2
            WHERE
                tu.tournament_id = $1
                AND (tup.is_invalid IS NULL OR tup.is_invalid = false)
            GROUP BY
                tu.id,
                au.username,
                au.profile_url
            ",
        )
        .bind(tournament_id)
        .bind(competition_id)
        .map(|row: sqlx::postgres::PgRow| LeaderboardEntry {
            tournament_user_id: row.get::<i64, _>("tournament_user_id") as u64,
            display_name: row.get("username"),
            avatar: row.get("profile_url"),
            points: row.try_get("points").unwrap_or(0.0),
            event_wins: row.get("exact_picks"),
            ordinal: row.get("ordinal"),
        })
        .fetch_all(&pool)
        .await?;

        return Ok(res);
    }

    pub async fn fetch_shotcaller_leaderboard(
        tournament_id: i64,
        competition_id: i64,
    ) -> Result<Vec<LeaderboardEntry>, Error> {
        let pool = DataClient::connect().await?;

        let res = sqlx::query(
            "
            SELECT
                tu.id as tournament_user_id,
                au.username,
                au.profile_url,
                sum(s.points) AS points,
                COUNT(*) FILTER (WHERE s.points = 100) AS exact_picks,
                RANK() OVER (
                    ORDER BY COALESCE(SUM(s.points), 0::double precision) DESC,
                    COUNT(*) FILTER (WHERE s.points = 100) DESC
                ) AS ordinal
            FROM tournament_users tu
                JOIN app_user au
                    ON au.id = tu.user_id
                LEFT JOIN tournament_user_picks tup
                    ON tup.tournament_user_id = tu.id
                LEFT JOIN workouts w
                    ON w.id = tup.workout_id
                LEFT JOIN score s
                    ON s.competitor_id = tup.competitor_id
                    AND s.ordinal = w.ordinal
                    AND s.competition_id = $2
            WHERE
                tu.tournament_id = $1
                AND (tup.is_invalid IS NULL OR tup.is_invalid = false)
            GROUP BY
                tu.id,
                au.username,
                au.profile_url
            ",
        )
        .bind(tournament_id)
        .bind(competition_id)
        .map(|row: sqlx::postgres::PgRow| LeaderboardEntry {
            tournament_user_id: row.get::<i64, _>("tournament_user_id") as u64,
            display_name: row.get("username"),
            avatar: row.get("profile_url"),
            points: row.try_get("points").unwrap_or(0.0),
            event_wins: row.get("exact_picks"),
            ordinal: row.get("ordinal"),
        })
        .fetch_all(&pool)
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
                competition_competitor.is_withdrawn,
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
                is_withdrawn: row.get("is_withdrawn"),
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
                AND tup.is_invalid = false
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

    pub async fn fetch_shotcaller_picks(
        tournament_id: i64,
        user_id: i64,
    ) -> Result<Vec<MatchupShotcallerPick>, Error> {
        let pool = DataClient::connect().await?;

        let res = sqlx::query(
            "
            SELECT
                tup.competitor_id,
                tup.tournament_position_id,
                tup.workout_id,
                c.first_name,
                c.last_name,
                cc.is_suspended,
                cc.is_cut,
                cc.is_withdrawn,
                s.points
            FROM
                tournament_users as tu
            JOIN
                tournament as t
                ON t.id = tu.tournament_id
            LEFT JOIN
                tournament_user_picks as tup
                ON tup.tournament_user_id = tu.id
            LEFT JOIN
                competitor as c
                ON c.id = tup.competitor_id
            LEFT JOIN
                competition_competitor cc
                ON cc.competitor_id = c.id AND t.competition_id = cc.competition_id
            LEFT JOIN
                workouts w
                ON w.id = tup.workout_id
            LEFT JOIN
                score s
                ON s.competitor_id = c.id AND s.competition_id = t.competition_id AND s.ordinal = w.ordinal
            WHERE
                tu.tournament_id = $1
                AND tu.id = $2
                AND tup.is_invalid = false
            ",
        )
        .bind(tournament_id)
        .bind(user_id)
        .map(|row: PgRow| MatchupShotcallerPick {
            competitor_id: row.get("competitor_id"),
            tournament_position_id: row.get("tournament_position_id"),
            workout_id: row.get("workout_id"),
            first_name: row.get("first_name"),
            last_name: row.get("last_name"),
            is_suspended: row.get("is_suspended"),
            is_cut: row.get("is_cut"),
            is_withdrawn: row.get("is_withdrawn"),
            points: row.try_get("points").unwrap_or(0.0),
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

    pub async fn fetch_pick_percentages(
        competition_id: i64,
    ) -> Result<HashMap<i64, Vec<PickPercentage>>, Error> {
        let pool = DataClient::connect().await?;

        let res = sqlx::query(
            "
            SELECT
                cpp.competitor_id,
                cpp.pick_percentage,
                cpp.workout_id
            FROM
                competitor_pick_percentages cpp
            WHERE
                cpp.competition_id = $1
            ",
        )
        .bind(competition_id as i64)
        .map(|row: PgRow| {
            (
                row.get("competitor_id"),
                row.get("pick_percentage"),
                row.get("workout_id"),
            )
        })
        .fetch_all(&pool)
        .await?;

        let mut result: HashMap<i64, Vec<PickPercentage>> = HashMap::new();

        for (competitor_id, pick_percentage, workout_id) in res {
            result
                .entry(competitor_id)
                .or_insert(vec![])
                .push(PickPercentage {
                    percentage: pick_percentage,
                    workout_id,
                });
        }

        return Ok(result);
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
                competition_competitor.position_id,
                competitor.first_name,
                competitor.last_name,
                competitor.gender_id,
                competition.is_active,
                competition.is_complete,
                competition_competitor.adp,
                positions.name as position_name
            FROM
                competition_competitor
            JOIN
                competitor
                ON competitor.id = competition_competitor.competitor_id
            JOIN
                competition
                ON competition.id = competition_competitor.competition_id
            LEFT JOIN
                positions
                ON competition_competitor.position_id = positions.id
            WHERE
                competition.id = $1
            ",
        )
        .bind(competition_id as i64)
        .map(|row: PgRow| {
            return LeagueAthletesResponse {
                competitor_id: row.get::<i64, _>("competitor_id") as u64,
                gender_id: row.get::<i64, _>("gender_id") as u64,
                first_name: row.get("first_name"),
                last_name: row.get("last_name"),
                adp: row.get::<f64, _>("adp"),
                pick_percentage: vec![],
                is_locked: row.get("is_active") || row.get("is_complete"),
                is_withdrawn: row.get("is_withdrawn"),
                is_cut: row.get("is_cut"),
                is_suspended: row.get("is_suspended"),
                position_id: row.get("position_id"),
                position: row.get("position_name"),
            };
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

    pub async fn fetch_positions(tournament_id: i64) -> Result<Vec<LeaguePosition>, Error> {
        let pool = DataClient::connect().await?;

        let res = sqlx::query(
            "
            SELECT
                tournament_positions.id as position_id,
                positions.name as position_name,
                positions.abbreviation as position_abbreviation,
                tournament_positions.ordinal as position_ordinal,
                positions.image_url as position_image_url,
                tournament_positions.allowed_positions
            FROM
                tournament_positions
            LEFT JOIN
                positions
                ON tournament_positions.position_id = positions.id
            WHERE
                tournament_positions.tournament_id = $1
            ",
        )
        .bind(tournament_id)
        .map(|row: PgRow| LeaguePosition {
            position_id: row.get("position_id"),
            name: row.get("position_name"),
            abbreviation: row.get("position_abbreviation"),
            ordinal: row.get("position_ordinal"),
            image_url: row.get("position_image_url"),
            allowed_positions: row.get("allowed_positions"),
        })
        .fetch_all(&pool)
        .await?;

        return Ok(res);
    }

    pub async fn fetch_user_leagues(
        user_league: &UserLeaguesRequest,
    ) -> Result<Vec<UserLeaguesResponse>, Error> {
        let pool = DataClient::connect().await?;

        let mut base_user_leagues: HashMap<i64, UserLeaguesResponse> = HashMap::new();

        let res = sqlx::query(
            "
            SELECT
                tournament_users.id as tournament_users_id,
                tournament_users.display_name,
                competition.name as competition_name,
                competition.id as competition_id,
                tournament.name as tournament_name,
                tournament.id as tournament_id,
                tournament.commissioner_id,
                tournament.logo,
                competition.logo as competition_logo,
                competition.locked_events,
                competition.is_active,
                competition.is_complete,
                tournament.tournament_type_id,
                tournament.pick_count,
                tournament_positions.id as position_id,
                positions.name as position_name,
                positions.abbreviation as position_abbreviation,
                tournament_positions.ordinal as position_ordinal,
                positions.image_url as position_image_url,
                tournament_positions.allowed_positions
            FROM
                tournament_users
            JOIN
                tournament
                ON tournament.id = tournament_users.tournament_id
            JOIN
                competition
                ON competition.id = tournament.competition_id
            LEFT JOIN
                tournament_positions
                ON tournament_positions.tournament_id = tournament.id
            LEFT JOIN
                positions
                ON tournament_positions.position_id = positions.id
            WHERE
                tournament_users.user_id = $1
                AND competition.id >= 28
            ",
        )
        .bind(user_league.user_id)
        .map(|row: PgRow| {
            let tu: i64 = row.get::<i64, _>("tournament_users_id");
            let existing_user_league = base_user_leagues.get(&tu);

            if existing_user_league.is_none() {
                base_user_leagues.insert(
                    row.get("tournament_users_id"),
                    UserLeaguesResponse {
                        tournament_user_id: tu as u64,
                        display_name: row.get::<Option<String>, _>("display_name"),
                        competition: row.get("competition_name"),
                        competition_id: row.get::<i64, _>("competition_id") as u64,
                        tournament: row.get("tournament_name"),
                        tournament_id: row.get::<i64, _>("tournament_id") as u64,
                        commissioner_id: row.get("commissioner_id"),
                        logo: row
                            .get::<Option<String>, _>("logo")
                            .unwrap_or(row.get("competition_logo")),
                        is_active: row.get("is_active"),
                        is_complete: row.get("is_complete"),
                        locked_events: row.get::<i64, _>("locked_events") as u64,
                        tournament_type_id: row.get::<i64, _>("tournament_type_id") as u64,
                        pick_count: row.get::<Option<i64>, _>("pick_count"),
                        positions: vec![],
                    },
                );
            }

            if row.try_get::<i64, _>("position_id").is_err() {
                return;
            }

            let new_position = LeaguePosition {
                position_id: row.get("position_id"),
                name: row.get("position_name"),
                abbreviation: row.get("position_abbreviation"),
                ordinal: row.get("position_ordinal"),
                image_url: row.get("position_image_url"),
                allowed_positions: row.get("allowed_positions"),
            };

            if let Some(user_league) = base_user_leagues.get_mut(&tu) {
                // Push the new position to the positions vector
                user_league.positions.push(new_position);
            } else {
                println!("Key not found in HashMap");
            }

            return;
        })
        .fetch_all(&pool)
        .await?;

        return Ok(base_user_leagues.values().cloned().collect());
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
                tournament_user_picks.workout_id,
                tournament_user_picks.tournament_position_id
            FROM
                tournament_user_picks
            JOIN
                competitor
                ON competitor.id = tournament_user_picks.competitor_id
            WHERE
                tournament_user_picks.tournament_user_id = $1
                AND tournament_user_picks.is_invalid = false
            ",
        )
        .bind(*tournament_user_id)
        .map(|row: PgRow| UserLeaguesPicksDataResponse {
            id: row.get("id"),
            competitor_id: row.get::<i64, _>("competitor_id") as u64,
            workout_id: row.get::<Option<i64>, _>("workout_id"),
            tournament_position_id: row.get::<i64, _>("tournament_position_id") as u64,
        })
        .fetch_all(&pool)
        .await?;

        return Ok(res);
    }

    pub async fn fetch_user_top_picks(
        tournament_user_id: &i64,
    ) -> Result<Vec<UserLeaguesTopPicksDataResponse>, Error> {
        let pool = DataClient::connect().await?;

        let res = sqlx::query(
            "
            SELECT
                tournament_user_picks.id,
                tournament_user_picks.competitor_id,
                tournament_user_picks.rank,
                competitor.gender_id,
                tournament_user_picks.tournament_position_id
            FROM
                tournament_user_picks
            JOIN
                competitor
                ON competitor.id = tournament_user_picks.competitor_id
            WHERE
                tournament_user_picks.tournament_user_id = $1
                AND tournament_user_picks.is_invalid = false
            ",
        )
        .bind(*tournament_user_id)
        .map(|row: PgRow| UserLeaguesTopPicksDataResponse {
            id: row.get("id"),
            competitor_id: row.get::<i64, _>("competitor_id") as u64,
            rank: row.get("rank"),
            gender_id: row.get("gender_id"),
            tournament_position_id: row.get::<i64, _>("tournament_position_id") as u64,
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
                tournament.logo,
                tournament.pick_count,
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
                logo: row.get("logo"),
                tournament_type_id: row.get::<i64, _>("tournament_type_id") as u64,
                is_private: row.get("is_private"),
                passcode: row.get("passcode"),
                entries: row.get::<i64, _>("entries") as u64,
                pick_count: row.get::<i64, _>("pick_count") as u64,
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

    pub async fn insert_tournament_user(tournament_id: i64, user_id: i64) -> Result<i64, Error> {
        let pool = DataClient::connect().await?;

        let res = sqlx::query(
            "
            INSERT INTO tournament_users (tournament_id, user_id)
            VALUES ($1, $2)
            RETURNING id
            ",
        )
        .bind(tournament_id)
        .bind(user_id)
        .fetch_one(&pool)
        .await?;

        let id = res.get("id");

        return Ok(id);
    }

    pub async fn insert_tournament_position(
        tournament_id: i64,
        position_id: i64,
        ordinal: i64,
    ) -> Result<(), Error> {
        let pool = DataClient::connect().await?;

        sqlx::query(
            "
            INSERT INTO tournament_positions(tournament_id, position_id, ordinal, allowed_positions)
            VALUES ($1, $2, $3, $4);
            ",
        )
        .bind(tournament_id)
        .bind(position_id)
        .bind(ordinal)
        .bind(if position_id < 5 {
            Some(vec![position_id])
        } else if position_id == 5 {
            Some(vec![1, 2, 3, 4])
        } else {
            None
        })
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

    pub async fn delete_tournament(tournament_id: i64) -> Result<(), Error> {
        let pool = DataClient::connect().await?;

        let _ = sqlx::query(
            "
            DELETE FROM tournament
            WHERE id = $1
            ",
        )
        .bind(tournament_id)
        .execute(&pool)
        .await?;

        return Ok(());
    }

    pub async fn delete_tournament_user(tournament_user_id: i64) -> Result<(), Error> {
        let pool = DataClient::connect().await?;

        let _ = sqlx::query(
            "
            DELETE FROM tournament_users
            WHERE id = $1
            ",
        )
        .bind(tournament_user_id)
        .execute(&pool)
        .await?;

        return Ok(());
    }

    pub async fn delete_tournament_users(tournament_id: i64) -> Result<(), Error> {
        let pool = DataClient::connect().await?;

        let _ = sqlx::query(
            "
            DELETE
            FROM tournament_users
            WHERE tournament_users.tournament_id = $1
            ",
        )
        .bind(tournament_id)
        .execute(&pool)
        .await?;

        return Ok(());
    }

    pub async fn delete_tournament_positions(tournament_id: i64) -> Result<(), Error> {
        let pool = DataClient::connect().await?;

        let _ = sqlx::query(
            "
            DELETE
            FROM tournament_positions
            WHERE tournament_positions.tournament_id = $1
            ",
        )
        .bind(tournament_id)
        .execute(&pool)
        .await?;

        return Ok(());
    }

    pub async fn delete_tournament_picks(tournament_id: i64) -> Result<(), Error> {
        let pool = DataClient::connect().await?;

        let _ = sqlx::query(
            "
            DELETE
            FROM tournament_user_picks
            USING tournament_users
            WHERE
                tournament_users.id = tournament_user_picks.tournament_user_id
                AND tournament_users.tournament_id = $1
            ",
        )
        .bind(tournament_id)
        .execute(&pool)
        .await?;

        return Ok(());
    }

    pub async fn delete_tournament_user_picks(tournament_user_id: i64) -> Result<(), Error> {
        let pool = DataClient::connect().await?;

        let _ = sqlx::query(
            "
            DELETE FROM tournament_user_picks
            USING tournament_users
            WHERE
                tournament_users.id = tournament_user_picks.tournament_user_id
                AND tournament_users.id = $1
            ",
        )
        .bind(tournament_user_id)
        .execute(&pool)
        .await?;

        return Ok(());
    }

    pub async fn update_pick_competitor(
        tournament_user_pick_id: i64,
        competitor_id: i64,
    ) -> Result<(), Error> {
        let pool = DataClient::connect().await?;

        sqlx::query(
            "
            UPDATE tournament_user_picks
            SET competitor_id = $2, last_updated = $3
            WHERE id = $1
            ",
        )
        .bind(tournament_user_pick_id)
        .bind(competitor_id)
        .bind(format!("{}", chrono::Utc::now()))
        .execute(&pool)
        .await?;

        return Ok(());
    }

    pub async fn swap_user_league_pick(
        previous_pick_id: i64,
        next_pick_id: i64,
    ) -> Result<(), Error> {
        let pool = DataClient::connect().await?;

        sqlx::query(
            "
            UPDATE tournament_user_picks
            SET tournament_user_id = $3, last_updated = $4
            WHERE tournament_user_id = $1 AND competitor_id = $2
            ",
        )
        .bind(previous_pick_id)
        .bind(next_pick_id)
        .bind(format!("{}", chrono::Utc::now()))
        .execute(&pool)
        .await?;

        return Ok(());
    }

    pub async fn insert_top_user_league_pick(
        tournament_user_id: i64,
        competitor_id: i64,
        rank: i64,
        tournament_position_id: i64,
    ) -> Result<(), Error> {
        let pool = DataClient::connect().await?;

        sqlx::query(
            "
            INSERT INTO tournament_user_picks (tournament_user_id, competitor_id, rank, tournament_position_id, last_updated)
            VALUES ($1, $2, $3, $4, $5)
            ",
        )
            .bind(tournament_user_id)
            .bind(competitor_id)
            .bind(rank)
            .bind(tournament_position_id)
            .bind(format!("{}", chrono::Utc::now()))
            .execute(&pool)
            .await?;

        return Ok(());
    }

    pub async fn insert_user_league_pick(
        tournament_user_id: i64,
        competitor_id: i64,
        workout_id: i64,
        tournament_position_id: i64,
    ) -> Result<(), Error> {
        let pool = DataClient::connect().await?;

        sqlx::query(
            "
            INSERT INTO tournament_user_picks (tournament_user_id, competitor_id, workout_id, tournament_position_id, last_updated)
            VALUES ($1, $2, $3, $4, $5)
            ",
        )
        .bind(tournament_user_id)
        .bind(competitor_id)
        .bind(workout_id)
        .bind(tournament_position_id)
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
            (competition_id, name, tournament_type_id, is_private, passcode, commissioner_id, pick_count, logo)
            VALUES
                ($1, $2, $3, $4, $5, $6, $7, $8)
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
        .bind(tournament.pick_count.unwrap_or(0i64))
            .bind(if tournament.tournament_type_id == 1 { "https://storage.googleapis.com/heat1-assets-pub/tournament/Top%2010%20Heat%201.png" } else { "https://storage.googleapis.com/heat1-assets-pub/tournament/Heat1%20shotcaller.png"})
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

    pub async fn fetch_all_competition_competitor_ids(
        competition_id: i64,
    ) -> Result<Vec<i64>, Error> {
        let pool = DataClient::connect().await?;

        let res = sqlx::query(
            "
            SELECT
                competition_competitor.competitor_id
            FROM
                competition_competitor
            WHERE
                competition_competitor.competition_id = $1
            ",
        )
        .bind(competition_id)
        .map(|row: PgRow| row.get("competitor_id"))
        .fetch_all(&pool)
        .await?;

        return Ok(res);
    }

    pub async fn fetch_tournament_pick_count(
        tournament_id: i64,
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
                ON tournament.id = tournament_users.tournament_id AND tournament.id = $1
            JOIN
                competitor
                ON competitor.id = tournament_user_picks.competitor_id
            WHERE
                competitor.gender_id = $2
                AND tournament_user_picks.is_invalid = false
            ",
        )
        .bind(tournament_id)
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

    pub async fn fetch_competition_pick_count(
        competition_id: i64,
        workout_id: i64,
    ) -> Result<HashMap<i64, i64>, Error> {
        let pool = DataClient::connect().await?;

        let res = sqlx::query(
            "
            SELECT
                tournament_user_picks.competitor_id,
                COUNT(*) as count
            FROM
                tournament_user_picks
            JOIN
                tournament_users
                ON tournament_users.id = tournament_user_picks.tournament_user_id
            JOIN
                tournament
                ON tournament.id = tournament_users.tournament_id
            WHERE
                tournament.competition_id = $1
                AND tournament_user_picks.is_invalid = false
                AND tournament_user_picks.workout_id = $2
            GROUP BY
                tournament_user_picks.competitor_id
            ",
        )
        .bind(competition_id)
        .bind(workout_id)
        .map(|row: PgRow| {
            (
                row.get::<i64, _>("competitor_id"),
                row.get::<i64, _>("count"),
            )
        })
        .fetch_all(&pool)
        .await?;

        let mut result: HashMap<i64, i64> = HashMap::new();

        for (competitor_id, count) in res {
            result.entry(competitor_id).or_insert(count);
        }

        return Ok(result);
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
                AND tournament_user_picks.is_invalid = false
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

    pub async fn fetch_competition_entries(
        competition_id: i64,
        workout_id: i64,
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
                ON tournament.id = tournament_users.tournament_id
            WHERE
                tournament.competition_id = $1
                AND tournament.tournament_type_id = 2
                AND tournament_user_picks.workout_id = $2
            ",
        )
        .bind(competition_id)
        .bind(workout_id)
        .map(|row: PgRow| row.get("count"))
        .fetch_one(&pool)
        .await?;

        return Ok(res);
    }

    pub async fn fetch_tournament_entries_new(
        tournament_id: i64,
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
                ON tournament.id = tournament_users.tournament_id AND tournament.id = $1
            JOIN
                competitor
                ON competitor.id = tournament_user_picks.competitor_id
            WHERE
                competitor.gender_id = $2
                AND tournament.tournament_type_id = 1
            ",
        )
        .bind(tournament_id)
        .bind(gender_id)
        .map(|row: sqlx::postgres::PgRow| row.get("count"))
        .fetch_one(&pool)
        .await?;

        return Ok(res);
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

    pub async fn update_competitor_pick_percentage(
        competitor_id: i64,
        competition_id: i64,
        workout_id: i64,
        pick_percentage: f64,
    ) -> Result<(), Error> {
        let pool = DataClient::connect().await?;

        sqlx::query(
            "
            INSERT INTO competitor_pick_percentages (competitor_id, competition_id, workout_id, pick_percentage)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT (competitor_id, competition_id, workout_id)
            DO UPDATE SET pick_percentage = EXCLUDED.pick_percentage
            ",
        )
        .bind(competitor_id)
        .bind(competition_id)
        .bind(workout_id)
        .bind(pick_percentage)
        .execute(&pool)
        .await?;

        return Ok(());
    }
    pub async fn update_competitor_adp(
        competitor_id: i64,
        competition_id: i64,
        adp: f64,
    ) -> Result<(), Error> {
        let pool = DataClient::connect().await?;

        sqlx::query(
            "
            UPDATE competition_competitor
            SET adp = $3
            WHERE competitor_id = $1 AND competition_id = $2
            ",
        )
        .bind(competitor_id)
        .bind(competition_id)
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
