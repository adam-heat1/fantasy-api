use crate::handlers::league::request_models::{InsertScoresRequest, WorkoutPredictionRequest};
use crate::{
    data::constants::ntfy,
    handlers::league::request_models::{
        CompetitionWorkoutRequest, CreateLeague, CreatePickRequest, JoinLeague,
        LeaderboardMatchupRequest, LeagueAthletes, LeagueLeaderboardRequest, OpenLeague,
        UserLeaguePicksRequest, UserLeaguesRequest,
    },
    services::league::LeagueService,
    utils::notification::spawn_notification,
};
use actix_web::{
    get, post, put,
    web::{Json, Path, Query, ServiceConfig},
    HttpResponse, Responder,
};
use validator::Validate;

pub fn configure(config: &mut ServiceConfig) {
    config
        .service(get_open_leagues)
        .service(get_league_athletes)
        .service(join_league)
        .service(get_user_league_picks)
        .service(get_shot_caller_picks)
        .service(get_user_leagues)
        .service(get_league_leaderboard)
        .service(get_leaderboard_matchup)
        .service(get_workout_prediction)
        .service(save_user_league_pick)
        .service(create_league)
        .service(update_scores)
        .service(unlock_workout)
        .service(lock_workout)
        .service(update_adp);
}

#[get("/open")]
pub(crate) async fn get_open_leagues(req: Query<OpenLeague>) -> impl Responder {
    if req.0.validate().is_err() {
        let message = format!("get_open_leagues: -> {:?}", req.validate().unwrap_err());
        spawn_notification(ntfy::ERROR.to_string(), message);

        return HttpResponse::BadRequest().body("Invalid open leagues request");
    }

    let user_id = &req.user_id;
    let competition_id = &req.competition_id;

    LeagueService::get_open_leagues(competition_id, user_id)
        .await
        .map_or_else(
            |e| {
                let error_message = format!(
                    "get_open_leagues: {} - {}: -> {:?}",
                    competition_id, user_id, e
                );
                spawn_notification(ntfy::ERROR.to_string(), error_message);

                HttpResponse::InternalServerError().body("Error getting open leagues")
            },
            |leagues| HttpResponse::Ok().json(leagues),
        )
}

#[get("/athletes")]
pub(crate) async fn get_league_athletes(req: Query<LeagueAthletes>) -> impl Responder {
    if req.0.validate().is_err() {
        let message = format!("get_league_athletes: -> {:?}", req.validate().unwrap_err());
        spawn_notification(ntfy::ERROR.to_string(), message);

        return HttpResponse::BadRequest().body("Invalid league athletes request");
    }

    let competition_id = &req.competition_id;

    LeagueService::get_league_athletes(competition_id)
        .await
        .map_or_else(
            |e| {
                let error_message = format!("get_league_athletes: {}: -> {:?}", competition_id, e);
                spawn_notification(ntfy::ERROR.to_string(), error_message);

                HttpResponse::InternalServerError().body("Error getting league athletes")
            },
            |leagues| HttpResponse::Ok().json(leagues),
        )
}

#[get("/user")]
pub(crate) async fn get_user_leagues(req: Query<UserLeaguesRequest>) -> impl Responder {
    if req.0.validate().is_err() {
        let message = format!("get_user_leagues: -> {:?}", req.validate().unwrap_err());
        spawn_notification(ntfy::ERROR.to_string(), message);

        return HttpResponse::BadRequest().body("Invalid user leagues request");
    }

    LeagueService::get_user_leagues(&req.0).await.map_or_else(
        |e| {
            let error_message = format!("get_user_leagues: {:?}: -> {:?}", &req.0, e);
            spawn_notification(ntfy::ERROR.to_string(), error_message);

            HttpResponse::InternalServerError().body("Error getting user leagues")
        },
        |leagues| HttpResponse::Ok().json(leagues),
    )
}

#[post("/pick")]
pub(crate) async fn save_user_league_pick(req: Json<CreatePickRequest>) -> impl Responder {
    LeagueService::save_user_league_pick(&req.0)
        .await
        .map_or_else(
            |e| {
                let error_message = format!(
                    "get_user_league_picks: {:?}: -> {:?}",
                    req.tournament_user_id, e
                );
                spawn_notification(ntfy::ERROR.to_string(), error_message);

                HttpResponse::InternalServerError().finish()
            },
            |leagues| HttpResponse::Ok().finish(),
        )
}

#[get("/picks/{userTournamentId}")]
pub(crate) async fn get_user_league_picks(req: Path<UserLeaguePicksRequest>) -> impl Responder {
    LeagueService::get_user_league_picks(&req.user_tournament_id)
        .await
        .map_or_else(
            |e| {
                let error_message = format!(
                    "get_user_league_picks: {:?}: -> {:?}",
                    req.user_tournament_id, e
                );
                spawn_notification(ntfy::ERROR.to_string(), error_message);

                HttpResponse::InternalServerError().body("Error getting user league picks")
            },
            |leagues| HttpResponse::Ok().json(leagues),
        )
}

#[get("/picks/shotcaller/{userTournamentId}")]
pub(crate) async fn get_shot_caller_picks(req: Path<UserLeaguePicksRequest>) -> impl Responder {
    LeagueService::get_shot_caller_picks(&req.user_tournament_id)
        .await
        .map_or_else(
            |e| {
                let error_message = format!(
                    "get_shot_caller_picks: {:?}: -> {:?}",
                    req.user_tournament_id, e
                );
                spawn_notification(ntfy::ERROR.to_string(), error_message);

                HttpResponse::InternalServerError().body("Error getting shot caller picks")
            },
            |leagues| HttpResponse::Ok().json(leagues),
        )
}

#[get("/{tournamentId}/leaderboard")]
pub(crate) async fn get_league_leaderboard(req: Path<LeagueLeaderboardRequest>) -> impl Responder {
    LeagueService::get_league_leaderboard(&req.tournament_id)
        .await
        .map_or_else(
            |e| {
                let error_message = format!(
                    "get_league_leaderboard: {:?}: -> {:?}",
                    req.tournament_id, e
                );
                spawn_notification(ntfy::ERROR.to_string(), error_message);

                HttpResponse::InternalServerError().body("Error getting league leaderboard")
            },
            |leagues| HttpResponse::Ok().json(leagues),
        )
}

#[get("/{tournamentId}/leaderboard/{userId}/{competitorId}")]
pub(crate) async fn get_leaderboard_matchup(
    req: Path<LeaderboardMatchupRequest>,
) -> impl Responder {
    LeagueService::get_leaderboard_matchup(&req.tournament_id, &req.user_id, &req.competitor_id)
        .await
        .map_or_else(
            |e| {
                let error_message = format!(
                    "get_leaderboard_matchup: {:?}: -> {:?}",
                    req.tournament_id, e
                );
                spawn_notification(ntfy::ERROR.to_string(), error_message);

                HttpResponse::InternalServerError().body("Error getting leaderboard matchup")
            },
            |matchup| HttpResponse::Ok().json(matchup),
        )
}

#[get("/prediction/{competitionId}/{ordinal}")]
pub(crate) async fn get_workout_prediction(req: Path<WorkoutPredictionRequest>) -> impl Responder {
    LeagueService::get_workout_prediction(&req.competition_id, &req.ordinal)
        .await
        .map_or_else(
            |e| {
                let error_message =
                    format!("get_workout_prediction: {:?}: -> {:?}", req.ordinal, e);
                spawn_notification(ntfy::ERROR.to_string(), error_message);

                HttpResponse::InternalServerError().body("Error getting workout prediction")
            },
            |prediction| HttpResponse::Ok().json(prediction),
        )
}

#[post("/")]
pub(crate) async fn create_league(body: Json<CreateLeague>) -> impl Responder {
    if body.validate().is_err() {
        let message = format!("create_league: -> {:?}", body.validate().unwrap_err());
        spawn_notification(ntfy::ERROR.to_string(), message);

        return HttpResponse::BadRequest().body("Invalid create league request");
    }

    LeagueService::create_league(&body.0).await.map_or_else(
        |e| {
            let message = format!("create_league: -> {:?}", e);
            spawn_notification(ntfy::ERROR.to_string(), message);

            HttpResponse::InternalServerError().body("Error creating league")
        },
        |response| HttpResponse::Ok().json(response),
    )
}

#[post("/scores")]
pub(crate) async fn update_scores(body: Json<InsertScoresRequest>) -> impl Responder {
    if body.validate().is_err() {
        let message = format!("update_scores: -> {:?}", body.validate().unwrap_err());
        spawn_notification(ntfy::ERROR.to_string(), message);

        return HttpResponse::BadRequest().body("Invalid update scores request");
    }

    LeagueService::update_scores(&body.0).await.map_or_else(
        |e| {
            let message = format!("update_scores: -> {:?}", e);
            spawn_notification(ntfy::ERROR.to_string(), message);

            HttpResponse::InternalServerError().body("Error updating scores")
        },
        |_| HttpResponse::Ok().finish(),
    )
}

#[post("/join")]
pub(crate) async fn join_league(body: Json<JoinLeague>) -> impl Responder {
    if body.validate().is_err() {
        let message = format!("join_league: -> {:?}", body.validate().unwrap_err());
        spawn_notification(ntfy::ERROR.to_string(), message);

        return HttpResponse::BadRequest().body("Invalid join league request");
    }

    LeagueService::join_league(&body.0).await.map_or_else(
        |e| {
            let message = format!("join_league: -> {:?}", e);
            spawn_notification(ntfy::ERROR.to_string(), message);

            HttpResponse::InternalServerError().body("Error joining league")
        },
        |response| HttpResponse::Ok().json(response),
    )
}

#[put("/{competitionId}/{ordinal}/unlock")]
pub(crate) async fn unlock_workout(req: Path<CompetitionWorkoutRequest>) -> impl Responder {
    LeagueService::unlock_workout(req.competition_id, req.ordinal)
        .await
        .map_or_else(
            |e| {
                let message = format!("unlock_workout: -> {:?}", e);
                spawn_notification(ntfy::ERROR.to_string(), message);

                HttpResponse::InternalServerError().body("Error unlocking workout")
            },
            |_| HttpResponse::Ok().finish(),
        )
}

#[put("/{competitionId}/{ordinal}/lock")]
pub(crate) async fn lock_workout(req: Path<CompetitionWorkoutRequest>) -> impl Responder {
    LeagueService::lock_workout(req.competition_id, req.ordinal)
        .await
        .map_or_else(
            |e| {
                let message = format!("lock_workout: -> {:?}", e);
                spawn_notification(ntfy::ERROR.to_string(), message);

                HttpResponse::InternalServerError().body("Error locking workout")
            },
            |_| HttpResponse::Ok().finish(),
        )
}

#[post("/adp")]
pub(crate) async fn update_adp() -> impl Responder {
    LeagueService::update_adp().await.map_or_else(
        |e| {
            let message = format!("update_adp: -> {:?}", e);
            spawn_notification(ntfy::ERROR.to_string(), message);

            HttpResponse::InternalServerError().body("Error updating adp")
        },
        |_| HttpResponse::Ok().finish(),
    )
}
