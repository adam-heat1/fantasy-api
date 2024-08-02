use crate::handlers::props::request_models::{
    CreatePropPickRequest, GetUserPropEntriesRequest, PropMatchupRequest, PropStatusRequest,
};
use crate::handlers::props::response_models::{
    SelfVsWorldEventScore, SelfVsWorldLeadeerboardEntry,
};
use crate::{
    data::constants::ntfy, handlers::props::request_models::GetPropsRequest,
    services::props::PropsService, utils::notification::spawn_notification,
};
use actix_web::web::Json;
use actix_web::{
    get, post, put,
    web::{Path, ServiceConfig},
    HttpResponse, Responder,
};
use serde_json::json;

pub fn configure(config: &mut ServiceConfig) {
    config
        .service(get_competition_props)
        .service(get_user_active_prop_entries)
        .service(get_active_prop_leaderboard)
        .service(get_self_vs_world_leaderboard)
        .service(get_prop_matchup)
        .service(create_prop_pick)
        .service(increment_bracket_download)
        .service(activate_prop)
        .service(disactivate_prop)
        .service(complete_prop)
        .service(uncomplete_prop);
}

#[get("/picks/{competitionId}/{tournamentUserId}")]
pub async fn get_competition_props(path: Path<GetPropsRequest>) -> impl Responder {
    PropsService::get_competition_props(path.competition_id, path.tournament_user_id)
        .await
        .map_or_else(
            |e| {
                if e.to_string().to_lowercase().contains("no rows returned") {
                    return HttpResponse::NotFound().body("No props found");
                }
                let error_message = format!(
                    "Error fetching competition props: {}: -> {:?}",
                    &path.tournament_user_id, e
                );
                spawn_notification(ntfy::ERROR.to_string(), error_message);

                HttpResponse::InternalServerError().body("Error fetching competition props")
            },
            |props| HttpResponse::Ok().json(json!(props)),
        )
}

#[get("/active/{userId}")]
pub async fn get_user_active_prop_entries(path: Path<GetUserPropEntriesRequest>) -> impl Responder {
    PropsService::get_user_active_prop_entries(path.user_id)
        .await
        .map_or_else(
            |e| {
                let error_message = format!(
                    "Error fetching active user props: {}: -> {:?}",
                    &path.user_id, e
                );
                spawn_notification(ntfy::ERROR.to_string(), error_message);

                HttpResponse::InternalServerError().body("Error fetching active user props")
            },
            |props| HttpResponse::Ok().json(json!(props)),
        )
}

#[get("/leaderboard")]
pub async fn get_active_prop_leaderboard() -> impl Responder {
    PropsService::get_active_prop_leaderboard()
        .await
        .map_or_else(
            |e| {
                let error_message = format!("Error fetching active prop leaderboard: -> {:?}", e);
                spawn_notification(ntfy::ERROR.to_string(), error_message);

                HttpResponse::InternalServerError().body("Error fetching active prop leaderboard")
            },
            |leaderboard| HttpResponse::Ok().json(json!(leaderboard)),
        )
}

#[get("/leaderboard/selfvsworld")]
pub async fn get_self_vs_world_leaderboard() -> impl Responder {
    let result = vec![
        SelfVsWorldLeadeerboardEntry {
            index: 1,
            avatar: "https://heat1storage.blob.core.windows.net/self-vs-world/0I5A7783.jpg"
                .to_string(),
            display_name: "Jayson Hopper".to_string(),
            events: vec![SelfVsWorldEventScore {
                rank: 4,
                points: 4.0,
                ordinal: 1,
                label: "E1".to_string(),
                description: "28 Reps".to_string(),
            },
            SelfVsWorldEventScore{
                rank: 2,
                points: 2.0,
                ordinal: 2,
                label: "E2".to_string(),
                description: "8:19".to_string(),
            },
                         SelfVsWorldEventScore{
                             rank:1,
                             points: 1.0,
                             ordinal: 3,
                             label: "E3".to_string(),
                             description: "-".to_string(),
                         },
                         SelfVsWorldEventScore{
                             rank:2,
                             points: 2.0,
                             ordinal: 4,
                             label: "E4".to_string(),
                             description: "-".to_string(),
                         }],
            points: 9.0,
        },
        SelfVsWorldLeadeerboardEntry {
            index: 2,
            avatar: "https://heat1storage.blob.core.windows.net/self-vs-world/9A0A3224.jpg"
                .to_string(),
            display_name: "Colten Mertens".to_string(),
            events: vec![SelfVsWorldEventScore {
                rank: 2,
                points: 2.0,
                ordinal: 1,
                label: "E1".to_string(),
                description: "31 Reps".to_string(),
            },
                         SelfVsWorldEventScore{
                             rank: 3,
                             points: 3.0,
                             ordinal: 2,
                             label: "E2".to_string(),
                             description: "-".to_string(),
                         },
                         SelfVsWorldEventScore{
                             rank:4,
                             points: 4.0,
                             ordinal: 3,
                             label: "E3".to_string(),
                             description: "-".to_string(),
                         },
                         SelfVsWorldEventScore{
                             rank:1,
                             points: 1.0,
                             ordinal: 4,
                             label: "E4".to_string(),
                             description: "-".to_string(),
                         }
            ],
            points: 10.0,
        },
        SelfVsWorldLeadeerboardEntry {
            index: 2,
            avatar: "https://heat1storage.blob.core.windows.net/self-vs-world/0I5A8652.jpg"
                .to_string(),
            display_name: "Dallin Pepper".to_string(),
            events: vec![
                SelfVsWorldEventScore {
                    rank: 1,
                    points: 1.0,
                    ordinal: 1,
                    label: "E1".to_string(),
                    description: "31 Reps".to_string(),
                },
                SelfVsWorldEventScore{
                    rank: 4,
                    points: 4.0,
                    ordinal: 2,
                    label: "E2".to_string(),
                    description: "-".to_string(),
                },

                SelfVsWorldEventScore{
                    rank:2,
                    points: 2.0,
                    ordinal: 3,
                    label: "E3".to_string(),
                    description: "-".to_string(),
                },
                SelfVsWorldEventScore{
                    rank:3,
                    points: 3.0,
                    ordinal: 4,
                    label: "E4".to_string(),
                    description: "-".to_string(),
                }
            ],
            points: 10.0,
        },
        SelfVsWorldLeadeerboardEntry {
            index: 4,
            avatar: "https://heat1storage.blob.core.windows.net/self-vs-world/FEDD2278-9BF3-4C88-890D-5D4C4617DC1D_1_105_c.jpg".to_string(),
            display_name: "Taylor Self".to_string(),
            events: vec!
            [SelfVsWorldEventScore {
                rank: 3,
                points: 3.0,
                ordinal: 1,
                label: "E1".to_string(),
                description: "29 Reps".to_string(),
            },
             SelfVsWorldEventScore{
                 rank: 1,
                 points: 1.0,
                 ordinal: 2,
                 label: "E2".to_string(),
                 description: "7:59".to_string(),
             },
             SelfVsWorldEventScore{
                 rank:3,
                 points: 3.0,
                 ordinal: 3,
                 label: "E3".to_string(),
                 description: "-".to_string(),
             },
             SelfVsWorldEventScore{
                 rank:4,
                 points: 4.0,
                 ordinal: 4,
                 label: "E4".to_string(),
                 description: "-".to_string(),
             }
            ],
            points: 11.0,
        },
    ];

    HttpResponse::Ok().json(json!(result))
}

#[get("/matchup/{userId}/{competitorId}")]
pub async fn get_prop_matchup(path: Path<PropMatchupRequest>) -> impl Responder {
    PropsService::get_prop_matchup(&path.user_id, &path.competitor_id)
        .await
        .map_or_else(
            |e| {
                let error_message = format!("Error fetching prop matchup: -> {:?}", e);
                spawn_notification(ntfy::ERROR.to_string(), error_message);

                HttpResponse::InternalServerError().body("Error fetching prop matchup")
            },
            |matchup| HttpResponse::Ok().json(json!(matchup)),
        )
}

#[post("/pick")]
pub async fn create_prop_pick(body: Json<CreatePropPickRequest>) -> impl Responder {
    PropsService::create_prop_pick(&body.0).await.map_or_else(
        |e| {
            let error_message = format!("Error picking prop: {:?}: -> {:?}", &body, e);
            spawn_notification(ntfy::ERROR.to_string(), error_message);

            HttpResponse::InternalServerError().body("Error picking prop")
        },
        |_| HttpResponse::Ok().finish(),
    )
}

#[post("/bracket/download")]
pub async fn increment_bracket_download() -> impl Responder {
    PropsService::increment_bracket_download()
        .await
        .map_or_else(
            |_| {
                spawn_notification(
                    ntfy::ERROR.to_string(),
                    "Error incrementing bracket".to_string(),
                );

                HttpResponse::InternalServerError().body("Error incrementing bracket")
            },
            |_| HttpResponse::Ok().finish(),
        )
}

#[put("/active/{propBetId}")]
pub(crate) async fn activate_prop(req: Path<PropStatusRequest>) -> impl Responder {
    PropsService::update_bet_active_status(req.prop_bet_id, true)
        .await
        .map_or_else(
            |e| {
                let message = format!("activate_prop: -> {:?}", e);
                spawn_notification(ntfy::ERROR.to_string(), message);

                HttpResponse::InternalServerError().body("Error activating prop")
            },
            |_| HttpResponse::Ok().finish(),
        )
}

#[put("/inactive/{propBetId}")]
pub(crate) async fn disactivate_prop(req: Path<PropStatusRequest>) -> impl Responder {
    PropsService::update_bet_active_status(req.prop_bet_id, false)
        .await
        .map_or_else(
            |e| {
                let message = format!("disactivate_prop: -> {:?}", e);
                spawn_notification(ntfy::ERROR.to_string(), message);

                HttpResponse::InternalServerError().body("Error disactivating prop")
            },
            |_| HttpResponse::Ok().finish(),
        )
}

#[put("/complete/{propBetId}")]
pub(crate) async fn complete_prop(req: Path<PropStatusRequest>) -> impl Responder {
    PropsService::update_bet_complete_status(req.prop_bet_id, true)
        .await
        .map_or_else(
            |e| {
                let message = format!("activate_prop: -> {:?}", e);
                spawn_notification(ntfy::ERROR.to_string(), message);

                HttpResponse::InternalServerError().body("Error activating prop")
            },
            |_| HttpResponse::Ok().finish(),
        )
}

#[put("/uncomplete/{propBetId}")]
pub(crate) async fn uncomplete_prop(req: Path<PropStatusRequest>) -> impl Responder {
    PropsService::update_bet_complete_status(req.prop_bet_id, false)
        .await
        .map_or_else(
            |e| {
                let message = format!("uncomplete_prop: -> {:?}", e);
                spawn_notification(ntfy::ERROR.to_string(), message);

                HttpResponse::InternalServerError().body("Error uncompleting prop")
            },
            |_| HttpResponse::Ok().finish(),
        )
}
