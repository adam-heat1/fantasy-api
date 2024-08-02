use crate::handlers::league::request_models::{CreateTopPickRequest, SwapPickRequest};
use crate::handlers::league::response_models::{
    LeaderboardMatchupShotcallerResponse, LeaderboardShotcallerPicks, MatchupShotcallerDetail,
    MatchupShotcallerPick, PositionPicks, ShotCallerPicksBetaResponse,
    UserLeaguesTopPicksDataResponse,
};
use crate::{
    data::{constants::ntfy, models::tournament::Tournament},
    handlers::{
        league::{
            request_models::{
                CreateLeague, CreateShotCallerPickRequest, InsertScoresRequest, JoinLeague,
                UserLeaguesRequest,
            },
            response_models::{
                CompetitionLeaderboardResponse, CompetitorPick, LeaderboardEntry,
                LeaderboardMatchupResponse, LeaderboardPicks, LeaderboardResponse,
                LeaderboardTournamentUserData, LeagueAthletesResponse, MatchupDetail, MatchupPick,
                OpenLeagueResponse, PropBet, PropBetOption, UserLeaguesPicksResponse,
                UserLeaguesResponse, WorkoutMovement, WorkoutPredictionResponse, WorkoutResponse,
                WorkoutStage,
            },
        },
        props::response_models::{PropMatchupDetail, PropUserMatchup},
    },
    repositories::league::LeagueRepository,
    repositories::props::PropsRepository,
    utils::notification::spawn_notification,
};
use log::info;
use sqlx::testing::TestTermination;
use sqlx::Error;
use std::collections::HashMap;
use tokio::join;

pub struct LeagueService;

impl LeagueService {
    pub async fn get_open_leagues(
        competition_id: &u64,
        user_id: &u64,
    ) -> Result<Vec<OpenLeagueResponse>, String> {
        LeagueRepository::fetch_open_leagues(competition_id, user_id)
            .await
            .map_err(|e| {
                let error_message = format!(
                    "get_open_leagues: {} - {}: -> {:?}",
                    competition_id, user_id, e
                );
                spawn_notification(ntfy::ERROR.to_string(), error_message);
                return "Unable to get open leagues".to_string();
            })
    }

    pub async fn get_user_leagues(
        user_league: &UserLeaguesRequest,
    ) -> Result<Vec<UserLeaguesResponse>, Error> {
        LeagueRepository::fetch_user_leagues(user_league).await
    }

    pub async fn swap_pick(pick_request: &SwapPickRequest) -> Result<(), String> {
        let event_status =
            LeagueRepository::fetch_competition_tournament_status(pick_request.tournament_user_id)
                .await
                .map_err(|e| "Unable to get event details".to_string())?;

        if event_status.is_complete {
            return Err("Can't swap picks for a complete competition".to_string());
        }

        if event_status.is_active {
            return Err("Can't swap picks for an active competition".to_string());
        }

        let previous_competitor =
            LeagueRepository::fetch_pick_competitor(pick_request.previous_pick_id)
                .await
                .map_err(|e| "Unable to get previous pick".to_string())?;

        if pick_request.next_pick.next_pick_id.is_none() {
            LeagueRepository::delete_user_league_pick(pick_request.previous_pick_id)
                .await
                .map_err(|e| "Unable to delete pick".to_string())?;

            let err = LeagueRepository::insert_top_user_league_pick(
                pick_request.tournament_user_id,
                previous_competitor.competitor_id,
                pick_request.next_pick.rank.unwrap(),
                pick_request.next_pick.tournament_position_id.unwrap(),
            )
            .await
            .map_err(|e| "Unable to get previous pick".to_string())?;

            return if err.is_success() {
                Ok(())
            } else {
                Err("Unable to insert top user league pick".to_string())
            };
        }

        let next_competitor =
            LeagueRepository::fetch_pick_competitor(pick_request.next_pick.next_pick_id.unwrap())
                .await
                .map_err(|e| "Unable to update pick".to_string())?;

        LeagueRepository::update_pick_competitor(
            pick_request.previous_pick_id,
            next_competitor.competitor_id,
        )
        .await
        .map_err(|e| "Unable to update pick".to_string())?;

        LeagueRepository::update_pick_competitor(
            pick_request.next_pick.next_pick_id.unwrap(),
            previous_competitor.competitor_id,
        )
        .await
        .map_err(|e| "Unable to update pick".to_string())?;

        return Ok(());
    }

    pub async fn save_top_user_league_pick(
        pick_request: &CreateTopPickRequest,
    ) -> Result<(), Error> {
        let event_status =
            LeagueRepository::fetch_competition_tournament_status(pick_request.tournament_user_id)
                .await?;

        if event_status.is_complete {
            return Err(Error::Protocol(
                "Can't update picks for a complete competition".to_string(),
            ));
        }

        if event_status.is_active {
            return Err(Error::Protocol(
                "Can't update picks for an active competition".to_string(),
            ));
        }

        if pick_request.competitor_id == 0
            || pick_request.rank == 0
            || pick_request.tournament_position_id == 0
        {
            return Err(Error::Protocol("Invalid Top Pick Request".to_string()));
        }

        let gender_id =
            LeagueRepository::fetch_competitor_gender_id(pick_request.competitor_id).await?;

        let previous_pick = LeagueRepository::fetch_top_pick_id(
            pick_request.tournament_user_id,
            gender_id,
            pick_request.tournament_position_id,
        )
        .await?;

        if previous_pick.is_some() {
            LeagueRepository::delete_user_league_pick(previous_pick.unwrap()).await?;
        }

        LeagueRepository::insert_top_user_league_pick(
            pick_request.tournament_user_id,
            pick_request.competitor_id,
            pick_request.rank,
            pick_request.tournament_position_id,
        )
        .await?;

        return Ok(());
    }

    pub async fn save_user_league_pick(
        pick_request: &CreateShotCallerPickRequest,
    ) -> Result<(), Error> {
        let event_status =
            LeagueRepository::fetch_competition_tournament_status(pick_request.tournament_user_id)
                .await?;

        let workout = LeagueRepository::fetch_workout(pick_request.workout_id).await?;

        if event_status.is_complete {
            return Err(Error::Protocol(
                "Can't update picks for a complete competition".to_string(),
            ));
        }

        if event_status.tournament_type_id == 2 && (workout.is_active || workout.is_complete) {
            return Err(Error::Protocol(
                "Can't update picks for an active event".to_string(),
            ));
        }

        if pick_request.competitor_id == 0
            || pick_request.workout_id == 0
            || pick_request.tournament_position_id == 0
        {
            return Err(Error::Protocol(
                "Invalid ShotCaller Pick Request".to_string(),
            ));
        }

        let previous_pick = LeagueRepository::fetch_shot_caller_pick_id(
            pick_request.tournament_user_id,
            pick_request.workout_id,
            pick_request.tournament_position_id,
        )
        .await?;

        if previous_pick.is_some() {
            LeagueRepository::delete_user_league_pick(previous_pick.unwrap()).await?;
        }

        LeagueRepository::insert_user_league_pick(
            pick_request.tournament_user_id,
            pick_request.competitor_id,
            pick_request.workout_id,
            pick_request.tournament_position_id,
        )
        .await?;

        return Ok(());
    }

    pub async fn delete_tournament(
        tournament_id: i64,
        user_id: i64,
    ) -> Result<Vec<UserLeaguesResponse>, Error> {
        LeagueRepository::delete_tournament_picks(tournament_id).await?;
        LeagueRepository::delete_tournament_users(tournament_id).await?;
        LeagueRepository::delete_tournament_positions(tournament_id).await?;
        LeagueRepository::delete_tournament(tournament_id).await?;

        let user_leagues =
            LeagueRepository::fetch_user_leagues(&UserLeaguesRequest { user_id }).await?;

        return Ok(user_leagues);
    }
    pub async fn delete_tournament_user(
        tournament_user_id: i64,
        user_id: i64,
    ) -> Result<Vec<UserLeaguesResponse>, Error> {
        LeagueRepository::delete_tournament_user_picks(tournament_user_id).await?;
        LeagueRepository::delete_tournament_user(tournament_user_id).await?;

        let user_leagues =
            LeagueRepository::fetch_user_leagues(&UserLeaguesRequest { user_id }).await?;

        return Ok(user_leagues);
    }

    pub async fn delete_user_league_top_pick(tournament_user_pick_id: i64) -> Result<(), Error> {
        let event_status =
            LeagueRepository::fetch_competition_tournament_status_by_pick(tournament_user_pick_id)
                .await?;

        if event_status.is_complete {
            return Err(Error::Protocol(
                "Can't delete picks for a complete competition".to_string(),
            ));
        }

        if event_status.is_active {
            return Err(Error::Protocol(
                "Can't delete picks for an active event".to_string(),
            ));
        }

        LeagueRepository::delete_user_league_pick(tournament_user_pick_id).await?;

        return Ok(());
    }

    pub async fn delete_user_league_pick(tournament_user_pick_id: i64) -> Result<(), Error> {
        let event_status =
            LeagueRepository::fetch_competition_tournament_status_by_pick(tournament_user_pick_id)
                .await?;
        let workout = LeagueRepository::fetch_workout_by_pick(tournament_user_pick_id).await?;

        if event_status.is_complete {
            return Err(Error::Protocol(
                "Can't delete picks for a complete competition".to_string(),
            ));
        }

        if event_status.tournament_type_id == 1
            || (event_status.tournament_type_id == 2 && (workout.is_active || workout.is_complete))
        {
            return Err(Error::Protocol(
                "Can't delete picks for an active event".to_string(),
            ));
        }

        LeagueRepository::delete_user_league_pick(tournament_user_pick_id).await?;

        return Ok(());
    }

    // pub async fn get_league_leaderboard(tournament_id: &i64) -> Result<LeaderboardResponse, Error> {
    //     info!("get_league_leaderboard: {}", tournament_id);

    //     let metadata = LeagueRepository::fetch_competition(*tournament_id).await?;
    //     let tournament_users = LeagueRepository::fetch_tournament_users(*tournament_id).await?;
    //     let men_leaderboard =
    //         LeagueRepository::fetch_competition_leaderboard(metadata.competition_id as i64, 1)
    //             .await?;
    //     let women_leaderboard =
    //         LeagueRepository::fetch_competition_leaderboard(metadata.competition_id as i64, 2)
    //             .await?;

    //     let leaderboard = LeaderboardResponse {
    //         tournament: metadata.tournament_name,
    //         competition: metadata.competition_name,
    //         logo: metadata.competition_logo,
    //         locked_events: metadata.locked_events,
    //         leaderboard: if metadata.tournament_type_id == 1 {
    //             // Self::get_top_10_entries(metadata.competition_id as i64, *tournament_id).await?
    //         } else {
    //             Self::get_top_shot_caller_entries(
    //                 tournament_users,
    //                 men_leaderboard,
    //                 women_leaderboard,
    //                 metadata.locked_events as i64,
    //                 *tournament_id,
    //             )
    //             .await?
    //         },
    //     };

    //     Ok(leaderboard)
    // }

    pub async fn get_league_leaderboard_new(
        tournament_id: &i64,
    ) -> Result<LeaderboardResponse, Error> {
        info!("get_league_leaderboard: {}", tournament_id);

        let metadata = LeagueRepository::fetch_competition(*tournament_id).await?;
        // let tournament_users = LeagueRepository::fetch_tournament_users(*tournament_id).await?;
        // let men_leaderboard =
        //     LeagueRepository::fetch_competition_leaderboard(metadata.competition_id as i64, 1)
        //         .await?;
        // let women_leaderboard =
        //     LeagueRepository::fetch_competition_leaderboard(metadata.competition_id as i64, 2)
        //         .await?;

        let leaderboard = LeaderboardResponse {
            tournament: metadata.tournament_name,
            competition: metadata.competition_name,
            logo: metadata.competition_logo,
            locked_events: metadata.locked_events,
            leaderboard: if metadata.tournament_type_id == 1 {
                LeagueRepository::fetch_top_10_leaderboard(
                    *tournament_id,
                    metadata.competition_id as i64,
                )
                .await?
            } else {
                LeagueRepository::fetch_shotcaller_leaderboard(
                    *tournament_id,
                    metadata.competition_id as i64,
                )
                .await?
            },
        };

        Ok(leaderboard)
    }

    fn get_top_10_picks(
        tournament_type_id: i64,
        leaderboard: &HashMap<i64, CompetitionLeaderboardResponse>,
    ) -> Vec<MatchupPick> {
        let mut leaderboard_result = leaderboard
            .clone()
            .into_values()
            .collect::<Vec<CompetitionLeaderboardResponse>>();

        if tournament_type_id == 1 {
            leaderboard_result.sort_by(|a, b| a.placement.partial_cmp(&b.placement).unwrap());
        } else {
            //TODO: Fix
            leaderboard_result.sort_by(|a, b| a.placement.partial_cmp(&b.placement).unwrap());
        }
        leaderboard_result
            .into_iter()
            .map(|p: CompetitionLeaderboardResponse| MatchupPick {
                predicted_rank: p.placement as u64,
                rank: p.placement as u64,
                first_name: p.first_name.clone(),
                last_name: p.last_name.clone(),
                competitor_id: p.competitor_id.clone() as u64,
                points: if tournament_type_id == 1 { 10.0 } else { 100.0 },
                event_points: p.points,
                is_withdrawn: false,
                is_cut: false,
                is_suspended: false,
                is_final: false,
            })
            .collect()
    }

    fn get_matchup_picks(
        picks: Vec<LeaderboardPicks>,
        leaderboard: &HashMap<i64, CompetitionLeaderboardResponse>,
    ) -> Vec<MatchupPick> {
        picks
            .iter()
            .map(|p| {
                let competitor_leaderboard =
                    Self::get_competitor_leaderboard(&leaderboard, p.competitor_id);

                let mut points: f64 = 0.0;

                // if tournament_type_id == 1 {
                let current_rank = competitor_leaderboard.placement;
                if current_rank != 0 {
                    let mut rank_diff = p.rank - current_rank;

                    if rank_diff < 0 {
                        rank_diff = rank_diff * -1;
                    }

                    points = (10 - rank_diff) as f64;

                    if points < 0.0 {
                        points = 0.0;
                    }
                }
                // } else {
                //     points = competitor_leaderboard
                //         .finishes
                //         .get(p.rank as usize - 1)
                //         .unwrap_or(&0.0)
                //         .clone();
                // };

                // let withdrawn_ids = vec![&346866, &73659i64, &298928i64, &360720i64];
                // let is_withdrawn = withdrawn_ids
                //     .iter()
                //     .any(|w| w == *competitor_leaderboard.competition_id);
                // if competitor_leaderboard.competitor_id == 346866
                // // || competitor_leaderboard.competitor_id == 73659
                // // || competitor_leaderboard.competitor_id == 298928
                // // || competitor_leaderboard.competitor_id == 360720
                // {
                //     true
                // } else {
                //     false
                // };
                MatchupPick {
                    predicted_rank: p.rank as u64,
                    rank: competitor_leaderboard.placement as u64,
                    first_name: competitor_leaderboard.first_name.clone(),
                    last_name: competitor_leaderboard.last_name.clone(),
                    competitor_id: competitor_leaderboard.competitor_id.clone() as u64,
                    points,
                    event_points: competitor_leaderboard.points,
                    is_withdrawn: competitor_leaderboard.is_withdrawn,
                    is_cut: false,
                    is_suspended: false,
                    is_final: competitor_leaderboard.is_withdrawn,
                }
            })
            .collect()
    }

    pub async fn get_workout_prediction(
        competition_id: &i64,
        ordinal: &i64,
    ) -> Result<Vec<WorkoutPredictionResponse>, Error> {
        let prediction_counts =
            LeagueRepository::fetch_workout_prediction_count(*competition_id, *ordinal).await?;

        let men_picks: &i64 = prediction_counts.get(&1i64).unwrap_or(&1i64);
        let women_picks: &i64 = prediction_counts.get(&2i64).unwrap_or(&1i64);

        LeagueRepository::fetch_workout_picks(*competition_id, *ordinal, *men_picks, *women_picks)
            .await
    }

    pub async fn get_leaderboard_matchup(
        tournament_id: &i64,
        user_id: &i64,
        competitor_id: &i64,
    ) -> Result<LeaderboardMatchupResponse, Error> {
        info!(
            "get_leaderboard_matchup: {} - {} - {}",
            tournament_id, user_id, competitor_id
        );

        let metadata = LeagueRepository::fetch_competition(*tournament_id).await?;

        let tournament_users =
            LeagueRepository::fetch_matchup_users(*tournament_id, *user_id, *competitor_id).await?;

        let men_leaderboard =
            LeagueRepository::fetch_competition_leaderboard(metadata.competition_id as i64, 1)
                .await?;
        let women_leaderboard =
            LeagueRepository::fetch_competition_leaderboard(metadata.competition_id as i64, 2)
                .await?;

        let user_picks = tournament_users
            .iter()
            .find(|tu| tu.tournament_user_id as i64 == *user_id)
            .unwrap_or(&LeaderboardTournamentUserData {
                tournament_user_id: 0,
                display_name: "".to_string(),
                avatar: "".to_string(),
                men_competitor_ids: vec![],
                women_competitor_ids: vec![],
            })
            .clone();

        let competitor_picks = tournament_users
            .iter()
            .find(|tu| tu.tournament_user_id as i64 == *competitor_id)
            .unwrap_or(&LeaderboardTournamentUserData {
                tournament_user_id: 0,
                display_name: "".to_string(),
                avatar: "".to_string(),
                men_competitor_ids: vec![],
                women_competitor_ids: vec![],
            })
            .clone();

        let user_men_players =
            Self::get_matchup_picks(user_picks.men_competitor_ids.clone(), &men_leaderboard);

        let user_women_players =
            Self::get_matchup_picks(user_picks.women_competitor_ids.clone(), &women_leaderboard);
        let competitor_men_players = if *competitor_id == 0 {
            Self::get_top_10_picks(metadata.tournament_type_id as i64, &men_leaderboard)
        } else {
            Self::get_matchup_picks(
                competitor_picks.men_competitor_ids.clone(),
                &men_leaderboard,
            )
        };

        let competitor_women_players = if *competitor_id == 0 {
            Self::get_top_10_picks(metadata.tournament_type_id as i64, &women_leaderboard)
        } else {
            Self::get_matchup_picks(
                competitor_picks.women_competitor_ids.clone(),
                &women_leaderboard,
            )
        };

        let leaderboard = LeaderboardMatchupResponse {
            locked_events: metadata.locked_events,
            user_matchup: MatchupDetail {
                men_points: user_men_players.iter().map(|p| p.points).sum(),
                women_points: user_women_players.iter().map(|p| p.points).sum(),
                men_players: user_men_players,
                women_players: user_women_players,
            },
            competitor_matchup: MatchupDetail {
                men_points: competitor_men_players.iter().map(|p| p.points).sum(),
                women_points: competitor_women_players.iter().map(|p| p.points).sum(),
                men_players: competitor_men_players,
                women_players: competitor_women_players,
            },
        };

        Ok(leaderboard)
    }

    pub async fn get_shotcaller_leaderboard_matchup(
        tournament_id: &i64,
        user_id: &i64,
        competitor_id: &i64,
    ) -> Result<LeaderboardMatchupShotcallerResponse, Error> {
        let res = join!(
            LeagueRepository::fetch_workouts_by_tournament(*tournament_id),
            LeagueRepository::fetch_shotcaller_picks(*tournament_id, *user_id),
            LeagueRepository::fetch_shotcaller_picks(*tournament_id, *competitor_id),
            PropsRepository::fetch_prop_matchup(*user_id),
            PropsRepository::fetch_prop_matchup(*competitor_id),
        );

        let workouts = res.0.unwrap();
        let user_picks = res.1.unwrap();
        let competitor_picks = res.2.unwrap();
        let user_prop_picks = res
            .3
            .unwrap_or(PropUserMatchup {
                display_name: "".to_string(),
                avatar: "".to_string(),
                points: 0.0,
                event_wins: 0,
                picks: vec![],
            })
            .picks;

        let competitor_prop_picks = res
            .4
            .unwrap_or(PropUserMatchup {
                display_name: "".to_string(),
                avatar: "".to_string(),
                points: 0.0,
                event_wins: 0,
                picks: vec![],
            })
            .picks;

        Ok(LeaderboardMatchupShotcallerResponse {
            workouts,
            user_matchup: MatchupShotcallerDetail {
                points: user_picks.iter().map(|p| p.points).sum(),
                players: user_picks,
                prop_points: if user_prop_picks.len() > 0 {
                    user_prop_picks.iter().map(|p| p.points).sum()
                } else {
                    0.0
                },
                prop_picks: user_prop_picks,
            },

            competitor_matchup: MatchupShotcallerDetail {
                points: competitor_picks.iter().map(|p| p.points).sum(),
                players: competitor_picks,
                prop_points: if competitor_prop_picks.len() > 0 {
                    competitor_prop_picks.iter().map(|p| p.points).sum()
                } else {
                    0.0
                },
                prop_picks: competitor_prop_picks,
            },
        })
    }

    fn get_competitor_leaderboard(
        leaderboard: &HashMap<i64, CompetitionLeaderboardResponse>,
        competitor_id: i64,
    ) -> CompetitionLeaderboardResponse {
        leaderboard
            .get(&competitor_id)
            .unwrap_or(&CompetitionLeaderboardResponse {
                competitor_id: 0,
                competition_id: 0,
                gender_id: 0,
                first_name: "".to_string(),
                last_name: "".to_string(),
                points: 0.0,
                finishes: vec![],
                placement: 0,
                is_withdrawn: false,
            })
            .clone()
    }

    // async fn get_top_shot_caller_entries(
    //     tournament_users: Vec<LeaderboardTournamentUserData>,
    //     men_leaderboard: HashMap<i64, CompetitionLeaderboardResponse>,
    //     women_leaderboard: HashMap<i64, CompetitionLeaderboardResponse>,
    //     locked_events: i64,
    //     tournament_id: i64,
    // ) -> Result<Vec<LeaderboardEntry>, Error> {
    //     let prop_results = PropsRepository::fetch_active_prop_leaderboard(tournament_id).await?;

    //     let mut leaderboard_entries: Vec<LeaderboardEntry> = tournament_users
    //         .iter()
    //         .map(|tu| {
    //             let mut event_wins = 0;
    //             let men_points = tu
    //                 .men_competitor_ids
    //                 .iter()
    //                 .filter(|c| c.rank <= locked_events)
    //                 .map(|p| {
    //                     let points =
    //                         Self::get_competitor_leaderboard(&men_leaderboard, p.competitor_id)
    //                             .finishes
    //                             .get(p.rank as usize - 1)
    //                             .unwrap_or(&0.0)
    //                             .clone();

    //                     if points == 50.0 || points == 100.0 {
    //                         event_wins = event_wins + 1;
    //                     }

    //                     return points;
    //                 })
    //                 .sum();
    //             let women_points = tu
    //                 .women_competitor_ids
    //                 .iter()
    //                 .filter(|c| c.rank <= locked_events)
    //                 .map(|p| {
    //                     let points =
    //                         Self::get_competitor_leaderboard(&women_leaderboard, p.competitor_id)
    //                             .finishes
    //                             .get(p.rank as usize - 1)
    //                             .unwrap_or(&0.0)
    //                             .clone();

    //                     if points == 50.0 || points == 100.0 {
    //                         event_wins = event_wins + 1;
    //                     }
    //                     return points;
    //                 })
    //                 .sum();

    //             let prop_points: f64 = prop_results
    //                 .iter()
    //                 .filter(|p| p.tournament_user_id == tu.tournament_user_id as i64)
    //                 .map(|p| p.points)
    //                 .sum();

    //             LeaderboardEntry {
    //                 tournament_user_id: tu.tournament_user_id,
    //                 display_name: tu.display_name.clone(),
    //                 avatar: tu.avatar.clone(),
    //                 event_wins,
    //                 points: men_points + women_points + prop_points,
    //                 ordinal: 0,
    //             }
    //         })
    //         .collect();

    //     leaderboard_entries.sort_by(|a, b| {
    //         if a.points == b.points {
    //             b.event_wins.partial_cmp(&a.event_wins).unwrap()
    //         } else {
    //             b.points.partial_cmp(&a.points).unwrap()
    //         }
    //     });

    //     // leaderboard_entries.sort_by(|a, b| b.points.partial_cmp(&a.points).unwrap());

    //     Ok(leaderboard_entries)
    // }

    // async fn get_top_10_entries(
    //     competition_id: i64,
    //     tournament_id: i64,
    // ) -> Result<Vec<LeaderboardEntry>, Error> {
    //     let tournament_users = LeagueRepository::fetch_tournament_users(tournament_id).await?;
    //     let men_leaderboard =
    //         LeagueRepository::fetch_competition_leaderboard(competition_id, 1).await?;
    //     let women_leaderboard =
    //         LeagueRepository::fetch_competition_leaderboard(competition_id, 2).await?;

    //     let mut leaderboard_entries: Vec<LeaderboardEntry> = tournament_users
    //         .iter()
    //         .map(|tu| {
    //             let mut event_wins = 0;

    //             let men_points = tu
    //                 .men_competitor_ids
    //                 .iter()
    //                 .map(|p| {
    //                     let current_rank =
    //                         Self::get_competitor_leaderboard(&men_leaderboard, p.competitor_id)
    //                             .placement;

    //                     if current_rank == 0 {
    //                         return 0.0;
    //                     }
    //                     let mut rank_diff = p.rank - current_rank;

    //                     if rank_diff < 0 {
    //                         rank_diff = rank_diff * -1;
    //                     }

    //                     let points = (10 - rank_diff) as f64;

    //                     if points < 0.0 {
    //                         return 0.0;
    //                     }

    //                     if rank_diff == 0 {
    //                         event_wins = event_wins + 1;
    //                     }

    //                     return points;
    //                 })
    //                 .sum();

    //             let women_points = tu
    //                 .women_competitor_ids
    //                 .iter()
    //                 .map(|p| {
    //                     let current_rank =
    //                         Self::get_competitor_leaderboard(&women_leaderboard, p.competitor_id)
    //                             .placement;
    //                     if current_rank == 0 {
    //                         return 0.0;
    //                     }
    //                     let mut rank_diff = p.rank - current_rank;

    //                     if rank_diff < 0 {
    //                         rank_diff = rank_diff * -1;
    //                     }

    //                     let points = (10 - rank_diff) as f64;

    //                     if points < 0.0 {
    //                         return 0.0;
    //                     }

    //                     if rank_diff == 0 {
    //                         event_wins = event_wins + 1;
    //                     }

    //                     return points;
    //                 })
    //                 .sum();
    //             LeaderboardEntry {
    //                 tournament_user_id: tu.tournament_user_id,
    //                 display_name: tu.display_name.clone(),
    //                 avatar: tu.avatar.clone(),
    //                 event_wins,
    //                 points: men_points + women_points,
    //                 ordinal: 0,
    //             }
    //         })
    //         .collect();

    //     leaderboard_entries.sort_by(|a, b| {
    //         if a.points == b.points {
    //             b.event_wins.partial_cmp(&a.event_wins).unwrap()
    //         } else {
    //             b.points.partial_cmp(&a.points).unwrap()
    //         }
    //     });
    //     // leaderboard_entries.sort_by(|a, b| b.points.partial_cmp(&a.points).unwrap());

    //     Ok(leaderboard_entries)
    // }

    fn get_gender_picks(
        picks: &Vec<UserLeaguesTopPicksDataResponse>,
        gender_id: i64,
    ) -> Vec<CompetitorPick> {
        picks
            .iter()
            .filter(|p| p.gender_id == gender_id)
            .map(|p| CompetitorPick {
                id: p.id,
                competitor_id: p.competitor_id,
                rank: p.rank,
            })
            .collect()
    }

    pub async fn get_user_league_picks(
        user_tournament_id: &i64,
    ) -> Result<UserLeaguesPicksResponse, Error> {
        let league_picks = LeagueRepository::fetch_user_top_picks(user_tournament_id).await?;

        let user_league_picks = UserLeaguesPicksResponse {
            tournament_user_id: *user_tournament_id as u64,
            men_picks: Self::get_gender_picks(&league_picks, 1),
            women_picks: Self::get_gender_picks(&league_picks, 2),
        };

        Ok(user_league_picks)
    }

    pub async fn get_shot_caller_picks_beta(
        user_tournament_id: &i64,
    ) -> Result<ShotCallerPicksBetaResponse, Error> {
        let competition_id =
            LeagueRepository::fetch_user_tournament_competition_id(user_tournament_id).await?;
        let props = LeagueRepository::fetch_shotcaller_props_by_competition(competition_id).await?;
        let prop_options =
            LeagueRepository::fetch_shotcaller_prop_options(competition_id, *user_tournament_id)
                .await?;
        let prop_option_picks = PropsRepository::fetch_prop_option_picks(competition_id).await?;
        let mut athletes = LeagueRepository::fetch_league_athletes(competition_id as u64).await?;
        let pick_percentages = LeagueRepository::fetch_pick_percentages(competition_id).await?;
        let competition_workouts = LeagueRepository::fetch_workouts(competition_id).await?;
        let workout_stages = LeagueRepository::fetch_workout_stages(competition_id).await?;
        let workout_stage_movements =
            LeagueRepository::fetch_workout_stage_movements(competition_id).await?;
        let league_picks = LeagueRepository::fetch_user_league_picks(user_tournament_id).await?;

        let workouts = competition_workouts
            .iter()
            .map(|w| {
                let stages = workout_stages
                    .iter()
                    .filter(|s| s.workout_id == w.id)
                    .map(|s| {
                        let movements = workout_stage_movements
                            .iter()
                            .filter(|m| m.workout_stage_id == s.id)
                            .map(|m| WorkoutMovement {
                                ordinal: m.ordinal,
                                name: m.name.clone(),
                            })
                            .collect::<Vec<WorkoutMovement>>();
                        WorkoutStage {
                            ordinal: s.ordinal,
                            time_cap: s.time_cap.clone(),
                            stage_type: s.stage_type.clone(),
                            movements: Some(movements),
                        }
                    })
                    .collect::<Vec<WorkoutStage>>();
                WorkoutResponse {
                    id: w.id,
                    name: w.name.clone(),
                    ordinal: w.ordinal,
                    start_time: w.start_time.clone(),
                    location: w.location.clone(),
                    description: w.description.clone(),
                    is_active: w.is_active,
                    is_complete: w.is_complete,
                    sponsor: w.sponsor.clone(),
                    sponsor_logo: w.sponsor_logo.clone(),
                    sponsor_logo_dark: w.sponsor_logo_dark.clone(),
                    sponsor_link: w.sponsor_link.clone(),
                    stages: Some(stages),
                }
            })
            .collect();

        athletes.iter_mut().for_each(|a| {
            let pick_percentage = pick_percentages
                .get(&(a.competitor_id as i64))
                .unwrap()
                .clone();
            a.pick_percentage = pick_percentage;
        });

        // athletes.sort_by(|a, b| a.adp.partial_cmp(&b.adp).unwrap_or(Ordering::Equal));

        let props_with_options = props
            .iter()
            .map(|p| {
                let opt = prop_options.get(&p.id).unwrap_or(&vec![]).clone();

                let total_picks: f64 = opt
                    .iter()
                    .map(|o| prop_option_picks.get(&o.id).unwrap_or(&0.0))
                    .sum();

                let percentage_opts = opt
                    .iter()
                    .map(|o| PropBetOption {
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

                PropBet {
                    id: p.id,
                    name: p.name.clone(),
                    start_time: p.start_time.clone(),
                    ordinal: p.ordinal,
                    is_active: p.is_active,
                    is_complete: p.is_complete,
                    description: p.description.clone(),
                    options: percentage_opts,
                }
            })
            .collect();

        let picks = league_picks
            .iter()
            .map(|p| PositionPicks {
                id: p.id,
                competitor_id: p.competitor_id,
                workout_id: p.workout_id.unwrap_or(0),
                position_id: p.tournament_position_id,
            })
            .collect();

        let user_league_picks = ShotCallerPicksBetaResponse {
            athletes,
            workouts,
            picks,
            props: props_with_options,
        };

        Ok(user_league_picks)
    }

    // pub async fn get_shot_caller_picks(
    //     user_tournament_id: &i64,
    // ) -> Result<ShotCallerPicksResponse, Error> {
    //     let competition_id =
    //         LeagueRepository::fetch_user_tournament_competition_id(user_tournament_id).await?;
    //     let props = LeagueRepository::fetch_shotcaller_props_by_competition(competition_id).await?;
    //     let prop_options =
    //         LeagueRepository::fetch_shotcaller_prop_options(competition_id, *user_tournament_id)
    //             .await?;
    //     let prop_option_picks = PropsRepository::fetch_prop_option_picks(competition_id).await?;
    //     let league_picks = LeagueRepository::fetch_user_league_picks(user_tournament_id).await?;
    //     let mut athletes = LeagueRepository::fetch_league_athletes(competition_id as u64).await?;
    //     let competition_workouts = LeagueRepository::fetch_workouts(competition_id).await?;
    //     let workout_stages = LeagueRepository::fetch_workout_stages(competition_id).await?;
    //     let workout_stage_movements =
    //         LeagueRepository::fetch_workout_stage_movements(competition_id).await?;
    //
    //     let workouts = competition_workouts
    //         .iter()
    //         .map(|w| {
    //             let stages = workout_stages
    //                 .iter()
    //                 .filter(|s| s.workout_id == w.id)
    //                 .map(|s| {
    //                     let movements = workout_stage_movements
    //                         .iter()
    //                         .filter(|m| m.workout_stage_id == s.id)
    //                         .map(|m| WorkoutMovement {
    //                             ordinal: m.ordinal,
    //                             name: m.name.clone(),
    //                         })
    //                         .collect::<Vec<WorkoutMovement>>();
    //                     WorkoutStage {
    //                         ordinal: s.ordinal,
    //                         time_cap: s.time_cap.clone(),
    //                         stage_type: s.stage_type.clone(),
    //                         movements: Some(movements),
    //                     }
    //                 })
    //                 .collect::<Vec<WorkoutStage>>();
    //             WorkoutResponse {
    //                 name: w.name.clone(),
    //                 ordinal: w.ordinal,
    //                 start_time: w.start_time.clone(),
    //                 location: w.location.clone(),
    //                 description: w.description.clone(),
    //                 is_active: w.is_active,
    //                 is_complete: w.is_complete,
    //                 stages: Some(stages),
    //             }
    //         })
    //         .collect();
    //
    //     athletes.sort_by(|a, b| a.adp.partial_cmp(&b.adp).unwrap_or(Ordering::Equal));
    //
    //     let props_with_options = props
    //         .iter()
    //         .map(|p| {
    //             let opt = prop_options.get(&p.id).unwrap_or(&vec![]).clone();
    //
    //             let total_picks: f64 = opt
    //                 .iter()
    //                 .map(|o| prop_option_picks.get(&o.id).unwrap_or(&0.0))
    //                 .sum();
    //
    //             let percentage_opts = opt
    //                 .iter()
    //                 .map(|o| PropBetOption {
    //                     id: o.id,
    //                     prop_bet_id: o.prop_bet_id,
    //                     name: o.name.clone(),
    //                     image_url: o.image_url.clone(),
    //                     points: o.points,
    //                     percentage: if total_picks == 0.0 {
    //                         0.0
    //                     } else {
    //                         (prop_option_picks.get(&o.id).unwrap_or(&0.0).clone() / total_picks
    //                             * 100.0)
    //                             .round()
    //                     },
    //                     is_picked: o.is_picked,
    //                 })
    //                 .collect();
    //
    //             PropBet {
    //                 id: p.id,
    //                 name: p.name.clone(),
    //                 start_time: p.start_time.clone(),
    //                 ordinal: p.ordinal,
    //                 is_active: p.is_active,
    //                 is_complete: p.is_complete,
    //                 options: percentage_opts,
    //             }
    //         })
    //         .collect();
    //
    //     let user_league_picks = ShotCallerPicksResponse {
    //         athletes,
    //         workouts,
    //         men_picks: Self::get_gender_picks(&league_picks, 1),
    //         women_picks: Self::get_gender_picks(&league_picks, 2),
    //         props: props_with_options,
    //     };
    //
    //     Ok(user_league_picks)
    // }

    pub async fn get_league_athletes(
        competition_id: &u64,
    ) -> Result<Vec<LeagueAthletesResponse>, Error> {
        LeagueRepository::fetch_league_athletes(*competition_id).await
    }
    pub async fn create_league(league: &CreateLeague) -> Result<UserLeaguesResponse, Error> {
        let new_league = Tournament {
            id: 0,
            competition_id: league.competition_id,
            name: league.name.clone(),
            logo: None,
            tournament_type_id: league.tournament_type_id,
            is_private: league.is_private,
            passcode: league.passcode.clone(),
            commissioner_id: league.user_id,
            entries: None,
            competition: None,
            tournament_type: None,
            pick_count: league.pick_count,
        };

        let league_id = LeagueRepository::insert_tournament(new_league).await?;
        LeagueRepository::insert_tournament_user(league_id as i64, league.user_id as i64).await?;

        if league.tournament_type_id == 1 {
            for i in 1..=league.pick_count.unwrap() {
                LeagueRepository::insert_tournament_position(league_id as i64, i + 5, i).await?;
            }
        } else {
            for i in 1..=5i64 {
                LeagueRepository::insert_tournament_position(league_id as i64, i, i).await?;
            }
        }

        let leagues = LeagueRepository::fetch_user_leagues(&UserLeaguesRequest {
            user_id: league.user_id as i64,
        })
        .await?;

        let league = leagues
            .iter()
            .find(|l| l.tournament_id == league_id)
            .unwrap();

        Ok(league.clone())
    }

    pub async fn update_scores(scores: &InsertScoresRequest) -> Result<(), Error> {
        let current_scores =
            LeagueRepository::fetch_scores(scores.competition_id, scores.ordinal).await?;

        for s in scores.scores.clone() {
            let existing_score = current_scores
                .iter()
                .find(|cs| cs.competitor_id == s.athlete_id);

            if existing_score.is_some() {
                LeagueRepository::update_score(existing_score.unwrap().id as i64, s.points).await?;
            } else {
                LeagueRepository::insert_score(
                    scores.competition_id,
                    s.athlete_id as i64,
                    scores.ordinal,
                    s.points,
                )
                .await?;
            }
        }

        LeagueRepository::refresh_competition_leaderboard().await
    }

    pub async fn join_league(league: &JoinLeague) -> Result<Vec<UserLeaguesResponse>, Error> {
        let is_user_in_league = LeagueRepository::fetch_is_user_in_league(league).await?;

        if !is_user_in_league {
            LeagueRepository::insert_tournament_user(league.tournament_id, league.user_id).await?;
        }

        let user_leagues = LeagueRepository::fetch_user_leagues(&UserLeaguesRequest {
            user_id: league.user_id,
        })
        .await?;

        Ok(user_leagues)
    }

    pub async fn unlock_workout(competition_id: i64, ordinal: i64) -> Result<(), Error> {
        let is_comp_locked = if ordinal == 1 { false } else { true };

        LeagueRepository::update_event(competition_id, is_comp_locked, ordinal - 1).await?;
        LeagueRepository::update_workout(competition_id, false, ordinal).await
    }

    pub async fn lock_workout(competition_id: i64, ordinal: i64) -> Result<(), Error> {
        LeagueRepository::update_event(competition_id, true, ordinal).await?;
        LeagueRepository::update_workout(competition_id, true, ordinal).await
    }

    async fn update_competition_gender_adp_new(
        competition_id: i64,
        gender_id: i64,
    ) -> Result<(), Error> {
        let tournaments = LeagueRepository::fetch_top_10_tournaments(competition_id).await?;
        let competitor_ids =
            LeagueRepository::fetch_competition_competitor_ids(competition_id, gender_id).await?;

        let mut competitor_pick_count: HashMap<i64, Vec<i64>> = HashMap::new();

        competitor_ids.iter().for_each(|c| {
            let is_picked = competitor_pick_count.get(c).unwrap_or(&vec![]).len() > 0;

            if !is_picked {
                competitor_pick_count.insert(*c, vec![]);
            }
        });

        for t in &tournaments {
            let tournament_entries =
                LeagueRepository::fetch_tournament_entries_new(t.id, gender_id).await?;

            let mut competitor_tournament_pick_count =
                LeagueRepository::fetch_tournament_pick_count(t.id, gender_id).await?;

            let pick_count = t.pick_count;
            let ceiling = pick_count + ((pick_count) / 2);

            for (k, v) in competitor_pick_count.iter_mut() {
                let tournament_picks = competitor_tournament_pick_count
                    .get(k)
                    .unwrap_or(&vec![])
                    .clone();

                let tournament_pick_count = tournament_picks.len() as i64;

                if tournament_pick_count > 0 {
                    v.extend(tournament_picks);
                }

                let pick_diff = tournament_entries - tournament_pick_count;

                if pick_diff > 0 {
                    for _ in 1..=pick_diff {
                        v.push(ceiling);
                    }
                }
            }
        }

        tournaments.iter().for_each(|t| {});

        for (k, v) in competitor_pick_count.iter_mut() {
            let pick_sum: i64 = v.iter().sum();
            let entries: f64 = v.len() as f64;

            let adp = pick_sum as f64 / entries;
            LeagueRepository::update_competitor_adp(*k, competition_id, adp).await?;
        }

        Ok(())
    }

    async fn update_competition_pick_percentage(
        competition_id: i64,
        workout_id: i64,
    ) -> Result<(), Error> {
        let competitor_ids =
            LeagueRepository::fetch_all_competition_competitor_ids(competition_id).await?;

        let competition_entries =
            LeagueRepository::fetch_competition_entries(competition_id, workout_id).await?;

        let mut competition_pick_count =
            LeagueRepository::fetch_competition_pick_count(competition_id, workout_id).await?;

        for c in competitor_ids {
            let competition_picks = competition_pick_count.get(&c).unwrap_or(&0).clone();

            let pick_percentage = competition_picks as f64 / competition_entries as f64 * 100.0;
            LeagueRepository::update_competitor_pick_percentage(
                c,
                competition_id,
                workout_id,
                pick_percentage,
            )
            .await?;
        }

        Ok(())
    }

    async fn update_competition_gender_adp(
        competition_id: i64,
        gender_id: i64,
    ) -> Result<(), Error> {
        let res = join!(
            LeagueRepository::fetch_competition_competitor_ids(competition_id, gender_id),
            LeagueRepository::fetch_competitor_pick_count(competition_id, gender_id),
            LeagueRepository::fetch_tournament_entries(competition_id, gender_id)
        );

        let competitor_ids = res.0.unwrap();
        let mut competitor_pick_count = res.1.unwrap();
        let tournament_entries = res.2.unwrap();

        competitor_ids.iter().for_each(|c| {
            let is_picked = competitor_pick_count.get(c).unwrap_or(&vec![]).len() > 0;

            if !is_picked {
                competitor_pick_count.insert(*c, vec![]);
            }
        });

        for (k, v) in competitor_pick_count.iter_mut() {
            let pick_sum: i64 = v.iter().sum();

            let competitor_pick_sum: f64 = if (v.len() as i64) < tournament_entries {
                pick_sum as f64 + (tournament_entries as f64 - v.len() as f64) * 40.0
            } else {
                pick_sum as f64
            };

            let adp = competitor_pick_sum / tournament_entries as f64;
            LeagueRepository::update_competitor_adp(*k, competition_id, adp).await?;
        }

        Ok(())
    }

    pub async fn update_adp() -> Result<(), Error> {
        let competition_id = 28i64;
        join!(
            Self::update_competition_gender_adp_new(competition_id, 1),
            Self::update_competition_gender_adp_new(competition_id, 2)
        );

        let workouts = LeagueRepository::fetch_workouts(competition_id).await?;
        for w in workouts {
            Self::update_competition_pick_percentage(competition_id, w.id).await?;
        }

        Ok(())
    }
}
