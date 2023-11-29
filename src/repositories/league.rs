use sqlx::{Error, Row};

use crate::{
    data::{data_client::DataClient, models::tournament::Tournament},
    handlers::league::response_models::OpenLeagueResponse,
};

pub struct LeagueRepository;

impl LeagueRepository {
    pub async fn fetch_open_leagues(
        competition_id: &u64,
        user_id: &u64,
    ) -> Result<Vec<OpenLeagueResponse>, Error> {
        let pool = DataClient::connect().await?;

        // (SELECT COUNT(*) FROM tournament_users WHERE tournament_users.tournament_id = tournament.id) as Entries
        // LEFT JOIN tournament_users
        // on tournament_users.tournament_id = tournament.id
        // AND tournament_users.user_id = $2
        // WHERE
        // tournament.competition_id = $1
        // AND tournament_users is null
        let res = sqlx::query(
            "
            SELECT
                id,
                competition_id,
                name,
                tournament_type_id,
                is_private,
                passcode,
                0 as Entries
            FROM
                tournament
            WHERE
                tournament.competition_id = $1
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
            entries: row.get::<i32, _>("entries") as u64,
        })
        .fetch_all(&pool)
        .await?;

        return Ok(res);
    }
    pub async fn insert_tournament_user(tournament_id: i64, user_id: i64) -> Result<(), Error> {
        let pool = DataClient::connect().await?;

        sqlx::query(
            "
            INSERT INTO
                tournament_users
            (tournament_id, user_id) 
            VALUES 
                ($1, $2)
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
