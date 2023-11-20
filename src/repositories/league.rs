use sqlx::Error;

use crate::data::data_client::DataClient;

pub struct LeagueRepository;

impl LeagueRepository {
    pub async fn create_app_user(tournament_id: i64, user_id: i64) -> Result<(), Error> {
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
