use crate::{
    handlers::{
        league::response_models::UserLeaguesResponse,
        props::response_models::PropBetsResponse,
        props::{
            request_models::CreatePropPickRequest,
            response_models::{
                PropBetOptions, PropLeaderboardResponse, PropMatchupResponse, PropUserMatchup,
            },
        },
    },
    repositories::{league::LeagueRepository, props::PropsRepository},
};
use sqlx::Error;

pub struct PropsService;

impl PropsService {
    pub async fn get_competition_props(
        competition_id: i64,
        tournament_user_id: i64,
    ) -> Result<Vec<PropBetsResponse>, Error> {
        let props = PropsRepository::fetch_props_by_competition(competition_id).await?;
        let options =
            PropsRepository::fetch_prop_options_by_competition(competition_id, tournament_user_id)
                .await?;

        let prop_option_picks = PropsRepository::fetch_prop_option_picks(competition_id).await?;

        let props_with_options = props
            .iter()
            .map(|p| {
                let opt = options.get(&p.id).unwrap_or(&vec![]).clone();

                let total_picks: f64 = opt
                    .iter()
                    .map(|o| prop_option_picks.get(&o.id).unwrap_or(&0.0))
                    .sum();

                let percentage_opts = opt
                    .iter()
                    .map(|o| PropBetOptions {
                        id: o.id,
                        prop_bet_id: o.prop_bet_id,
                        name: o.name.clone(),
                        image_url: o.image_url.clone(),
                        points: o.points,
                        percentage: if total_picks == 0.0 {
                            0.0
                        } else {
                            (prop_option_picks.get(&o.id).unwrap_or(&0.0).clone() / total_picks
                                * 100.0)
                                .round()
                        },
                        is_picked: o.is_picked,
                    })
                    .collect();

                PropBetsResponse {
                    id: p.id,
                    name: p.name.clone(),
                    start_time: p.start_time.clone(),
                    ordinal: p.ordinal,
                    is_active: p.is_active,
                    is_complete: p.is_complete,
                    workout_id: p.workout_id,
                    workout_name: p.workout_name.clone(),
                    workout_ordinal: p.workout_ordinal,
                    options: percentage_opts,
                }
            })
            .collect();

        Ok(props_with_options)
    }

    pub async fn get_user_active_prop_entries(user_id: i64) -> Result<UserLeaguesResponse, Error> {
        let tournament_id = 511i64;
        let user_props = PropsRepository::fetch_active_user_props(user_id, tournament_id).await?;

        if user_props.is_none() {
            LeagueRepository::insert_tournament_user(tournament_id, user_id).await?;
            let new_user_props =
                PropsRepository::fetch_active_user_props(user_id, tournament_id).await?;
            Ok(new_user_props.unwrap())
        } else {
            Ok(user_props.unwrap())
        }
    }

    pub async fn get_active_prop_leaderboard() -> Result<PropLeaderboardResponse, Error> {
        let tournament_id = 511i64;
        let metadata = LeagueRepository::fetch_competition(tournament_id).await?;
        let leaderboard_results =
            PropsRepository::fetch_active_prop_leaderboard(tournament_id).await?;

        Ok(PropLeaderboardResponse {
            tournament: metadata.tournament_name,
            competition: metadata.competition_name,
            logo: metadata.competition_logo,
            leaderboard: leaderboard_results,
        })
    }

    pub async fn get_prop_matchup(
        user_id: &i64,
        competitor_id: &i64,
    ) -> Result<PropMatchupResponse, Error> {
        let user_matchup = PropsRepository::fetch_prop_matchup(*user_id).await?;
        let competitor_matchup = if competitor_id == &0i64 {
            PropUserMatchup {
                display_name: "2024 Open".to_string(),
                avatar: "".to_string(),
                points: 0.0,
                event_wins: 0,
                picks: vec![],
            }
        } else {
            PropsRepository::fetch_prop_matchup(*competitor_id).await?
        };

        Ok(PropMatchupResponse {
            user_matchup,
            competitor_matchup,
        })
    }

    pub async fn create_prop_pick(prop_pick: &CreatePropPickRequest) -> Result<(), Error> {
        let prop = PropsRepository::fetch_prop_by_id(prop_pick.prop_id).await?;
        if prop.is_active || prop.is_complete {
            return Err(Error::Protocol(
                "Can't update picks for an active or complete competition".to_string(),
            ));
        }

        let user_pick =
            PropsRepository::fetch_user_pick(prop_pick.tournament_user_id, prop_pick.prop_id)
                .await?;

        if user_pick.is_none() {
            PropsRepository::create_user_pick(
                prop_pick.tournament_user_id,
                prop_pick.prop_option_id,
            )
            .await?;

            return Ok(());
        }

        let pick = user_pick.unwrap();

        if pick.prop_option_id == prop_pick.prop_option_id {
            return Ok(());
        }

        PropsRepository::update_user_pick(pick.id, prop_pick.prop_option_id).await
    }

    pub async fn increment_bracket_download() -> Result<(), Error> {
        PropsRepository::increment_bracket_counter().await
    }

    pub async fn update_bet_active_status(prop_bet_id: i64, is_active: bool) -> Result<(), Error> {
        PropsRepository::update_bet_active_status(prop_bet_id, is_active).await
    }

    pub async fn update_bet_complete_status(
        prop_bet_id: i64,
        is_complete: bool,
    ) -> Result<(), Error> {
        PropsRepository::update_bet_complete_status(prop_bet_id, is_complete).await
    }
}
