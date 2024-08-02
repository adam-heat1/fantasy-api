use crate::handlers::props::response_models::PropMatchupDetail;
use crate::handlers::{
    league::response_models::UserLeaguesResponse,
    props::response_models::{
        PropBetOptions, PropLeaderboardEntry, PropPickResponse, PropUserMatchup,
    },
};
use crate::{data::data_client::DataClient, handlers::props::response_models::PropBetsResponse};
use sqlx::{postgres::PgRow, Error, Row};
use std::collections::HashMap;

pub struct PropsRepository;

#[derive(sqlx::Type)]
#[sqlx(transparent, no_pg_array)]
struct Picks(Vec<(i64, f64, String, String, String, bool, bool, bool, String)>);

impl PropsRepository {
    pub async fn update_bet_active_status(prop_bet_id: i64, is_active: bool) -> Result<(), Error> {
        let pool = DataClient::connect().await?;

        let _res = sqlx::query("UPDATE prop_bets SET is_active = $2 WHERE id = $1")
            .bind(prop_bet_id)
            .bind(is_active)
            .execute(&pool)
            .await?;

        return Ok(());
    }

    pub async fn update_bet_complete_status(
        prop_bet_id: i64,
        is_complete: bool,
    ) -> Result<(), Error> {
        let pool = DataClient::connect().await?;

        let _res = sqlx::query("UPDATE prop_bets SET is_complete = $2 WHERE id = $1")
            .bind(prop_bet_id)
            .bind(is_complete)
            .execute(&pool)
            .await?;

        return Ok(());
    }

    pub async fn fetch_prop_by_id(prop_id: i64) -> Result<PropBetsResponse, Error> {
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
                prop_bets.is_complete
            FROM
                prop_bets
            WHERE
                prop_bets.id = $1
            ",
        )
        .bind(prop_id)
        .map(|row: PgRow| PropBetsResponse {
            id: row.get("id"),
            name: row.get("name"),
            start_time: row.get("start_time"),
            ordinal: row.get("ordinal"),
            is_active: row.get("is_active"),
            is_complete: row.get("is_complete"),
            workout_id: row.get("workout_id"),
            workout_name: "".to_string(),
            workout_ordinal: 0i64,
            options: vec![],
        })
        .fetch_one(&pool)
        .await?;

        return Ok(res);
    }

    pub async fn fetch_props_by_competition(
        competition_id: i64,
    ) -> Result<Vec<PropBetsResponse>, Error> {
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
                workouts.name as workout_name,
                workouts.ordinal as workout_ordinal
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
        .map(|row: PgRow| PropBetsResponse {
            id: row.get("id"),
            name: row.get("name"),
            start_time: row.get("start_time"),
            ordinal: row.get("ordinal"),
            is_active: row.get("is_active"),
            is_complete: row.get("is_complete"),
            workout_id: row.get("workout_id"),
            workout_name: row.get("workout_name"),
            workout_ordinal: row.get("workout_ordinal"),
            options: vec![],
        })
        .fetch_all(&pool)
        .await?;

        return Ok(res);
    }

    pub async fn fetch_prop_options_by_competition(
        competition_id: i64,
        tournament_user_id: i64,
    ) -> Result<HashMap<i64, Vec<PropBetOptions>>, Error> {
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
        .map(|row: PgRow| PropBetOptions {
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

        let mut result: HashMap<i64, Vec<PropBetOptions>> = HashMap::new();

        for r in res {
            result.entry(r.prop_bet_id).or_insert(vec![]).push(r);
        }

        return Ok(result);
    }

    pub async fn fetch_prop_option_picks(competition_id: i64) -> Result<HashMap<i64, f64>, Error> {
        let pool = DataClient::connect().await?;

        let res = sqlx::query(
            "
            SELECT
                prop_options.id,
                COUNT(prop_picks.prop_option_id) as count
            FROM
                prop_options
            JOIN
                prop_bets ON prop_bets.id = prop_options.prop_bet_id
            JOIN
                workouts ON workouts.id = prop_bets.workout_id
            JOIN
                prop_picks ON prop_picks.prop_option_id = prop_options.id
            WHERE
                prop_bets.is_hidden = false
                AND workouts.competition_id = $1
                AND prop_picks.is_valid = true
            GROUP BY
                prop_options.id
            ",
        )
        .bind(competition_id)
        .map(|row: PgRow| PropPickResponse {
            id: row.get("id"),
            prop_option_id: row.get("count"),
        })
        .fetch_all(&pool)
        .await?;

        let result = res
            .into_iter()
            .map(|s| (s.id, s.prop_option_id as f64))
            .collect();

        return Ok(result);
    }

    pub async fn fetch_active_user_props(
        user_id: i64,
        tournament_id: i64,
    ) -> Result<Option<UserLeaguesResponse>, Error> {
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
                tournament.commissioner_id,
                competition.logo,
                competition.locked_events,
                competition.is_active,
                competition.is_complete,
                tournament.tournament_type_id,
                tournament.pick_count
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
                AND tournament.id = $2
            ",
        )
        .bind(user_id)
        .bind(tournament_id)
        .map(|row: PgRow| UserLeaguesResponse {
            tournament_user_id: row.get::<i64, _>("tournament_users_id") as u64,
            display_name: row.get::<Option<String>, _>("display_name"),
            competition: row.get("competition_name"),
            competition_id: row.get::<i64, _>("competition_id") as u64,
            tournament: row.get("tournament_name"),
            tournament_id: row.get::<i64, _>("tournament_id") as u64,
            commissioner_id: row.get("commissioner_id"),
            logo: row.get("logo"),
            is_active: row.get("is_active"),
            is_complete: row.get("is_complete"),
            locked_events: row.get::<i64, _>("locked_events") as u64,
            tournament_type_id: row.get::<i64, _>("tournament_type_id") as u64,
            pick_count: row.get::<Option<i64>, _>("pick_count"),
            positions: vec![],
        })
        .fetch_optional(&pool)
        .await?;

        return Ok(res);
    }

    pub async fn fetch_active_prop_leaderboard(
        tournament_id: i64,
    ) -> Result<Vec<PropLeaderboardEntry>, Error> {
        let pool = DataClient::connect().await?;

        let res = sqlx::query(
            "
            SELECT
                tournament_users.id as tournament_users_id,
                tournament_users.display_name,
                app_user.profile_url,
                app_user.username,
                SUM(CASE WHEN prop_options.is_winner = true THEN prop_options.points ELSE 0 END) AS points,
                SUM(CASE WHEN prop_options.is_winner = true THEN 1 ELSE 0 END) as event_wins
            FROM
                tournament_users
            JOIN app_user
                ON app_user.id = tournament_users.user_id
            LEFT JOIN
                prop_picks
                ON prop_picks.tournament_user_id = tournament_users.id
            LEFT JOIN
                prop_options
                ON prop_options.id = prop_picks.prop_option_id
            WHERE
                tournament_users.tournament_id = $1
                AND (prop_picks.is_valid IS NULL OR prop_picks.is_valid = true)
            GROUP BY
                tournament_users.id,
                tournament_users.display_name,
                app_user.profile_url,
                app_user.username
            ORDER BY
                points DESC,
                event_wins DESC
            ",
        )
        .bind(tournament_id)
        .map(|row: PgRow| {
            let display_name = row.get::<Option<String>, _>("display_name");
            PropLeaderboardEntry {
                tournament_user_id: row.get("tournament_users_id"),
                display_name: if display_name.is_some() {
                    display_name.unwrap()
                } else {
                    row.get("username")
                },
                avatar: row.get("profile_url"),
                points: row.get("points"),
                event_wins: row.get("event_wins"),
            }
        })
        .fetch_all(&pool)
        .await?;

        return Ok(res);
    }

    pub async fn fetch_prop_matchup(tournament_user_id: i64) -> Result<PropUserMatchup, Error> {
        let pool = DataClient::connect().await?;

        let res = sqlx::query(
            "
            SELECT
                tournament_users.id as tournament_users_id,
                tournament_users.display_name,
                app_user.profile_url,
                app_user.username,
                SUM(CASE WHEN prop_options.is_winner = true THEN prop_options.points ELSE 0 END) AS points,
                SUM(CASE WHEN prop_options.is_winner = true THEN 1 ELSE 0 END) as event_wins,
                ARRAY_AGG((prop_bets.ordinal, prop_options.points, prop_bets.name, prop_options.name, prop_options.image_url, prop_options.is_winner, prop_bets.is_active, prop_bets.is_complete, workouts.name)) as picks
            FROM
                tournament_users
            JOIN app_user
                ON app_user.id = tournament_users.user_id
            LEFT JOIN
                prop_picks
                ON prop_picks.tournament_user_id = tournament_users.id
            LEFT JOIN
                prop_options
                ON prop_options.id = prop_picks.prop_option_id
            LEFT JOIN
                prop_bets
                ON prop_bets.id = prop_options.prop_bet_id
            LEFT JOIN
                workouts
                ON workouts.id = prop_bets.workout_id
            WHERE
                tournament_users.id = $1
                AND (prop_picks.is_valid IS NULL OR prop_picks.is_valid = true)
                AND prop_bets.is_hidden = false
            GROUP BY
                tournament_users.id,
                tournament_users.display_name,
                app_user.profile_url,
                app_user.username
            ",
        )
        .bind(tournament_user_id)
        .map(|row: PgRow| {
            let display_name = row.get::<Option<String>, _>("display_name");
            let picks = row.try_get::<Picks, _>("picks").unwrap_or(Picks(vec![])).0;

            PropUserMatchup {
                display_name: if display_name.is_some() {
                    display_name.unwrap()
                } else {
                    row.get("username")
                },
                avatar: row.get("profile_url"),
                points: row.get("points"),
                event_wins: row.get("event_wins"),
                picks: picks.iter().map(|p| PropMatchupDetail {
                    ordinal: p.0,
                    points: if p.5 == true {p.1} else { 0.0},
                    description: p.2.clone(),
                    name: p.3.clone(),
                    image_url: p.4.clone(),
                    is_locked: p.6 || p.7,
                    workout: p.8.clone(),
                    metadata: p.8.clone(),
                }).collect(),
            }
        })
        .fetch_one(&pool)
        .await?;

        return Ok(res);
    }

    pub async fn fetch_user_pick(
        tournament_user_id: i64,
        prop_id: i64,
    ) -> Result<Option<PropPickResponse>, Error> {
        let pool = DataClient::connect().await?;

        let res = sqlx::query(
            "
            SELECT
                prop_picks.id,
                prop_picks.prop_option_id
            FROM
                prop_picks
            JOIN
                prop_options
                ON prop_options.id = prop_picks.prop_option_id
            WHERE
                prop_picks.tournament_user_id = $1
                AND prop_options.prop_bet_id = $2
                AND prop_picks.is_valid = true
            ",
        )
        .bind(tournament_user_id)
        .bind(prop_id)
        .map(|row: PgRow| PropPickResponse {
            id: row.get("id"),
            prop_option_id: row.get("prop_option_id"),
        })
        .fetch_optional(&pool)
        .await?;

        return Ok(res);
    }

    pub async fn create_user_pick(
        tournament_user_id: i64,
        prop_option_id: i64,
    ) -> Result<(), Error> {
        let pool = DataClient::connect().await?;

        let _ = sqlx::query(
            "
            INSERT INTO prop_picks (tournament_user_id, prop_option_id)
            VALUES ($1, $2)
            ",
        )
        .bind(tournament_user_id)
        .bind(prop_option_id)
        .execute(&pool)
        .await?;

        Ok(())
    }

    pub async fn update_user_pick(id: i64, prop_option_id: i64) -> Result<(), Error> {
        let pool = DataClient::connect().await?;

        let _ = sqlx::query(
            "
            UPDATE prop_picks
            SET prop_option_id = $2, last_updated = now()
            WHERE id = $1
            ",
        )
        .bind(id)
        .bind(prop_option_id)
        .execute(&pool)
        .await?;

        Ok(())
    }

    pub async fn increment_bracket_counter() -> Result<(), Error> {
        let pool = DataClient::connect().await?;

        let _ = sqlx::query(
            "
            UPDATE bracket_counter
            SET counter = counter + 1
            WHERE id = 1
            ",
        )
        .execute(&pool)
        .await?;

        Ok(())
    }
}
