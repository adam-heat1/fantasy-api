use crate::data::constants::ntfy;
use crate::handlers::league::request_models::InsertScoresRequest;
use crate::handlers::league::response_models::{
    CompetitionLeaderboardResponse, LeaderboardMatchupResponse, LeaderboardPicks,
    LeaderboardTournamentUserData, MatchupDetail, MatchupPick, WorkoutPredictionResponse,
};
use crate::utils::notification::spawn_notification;
use crate::{
    data::models::tournament::Tournament,
    handlers::league::{
        request_models::{CreateLeague, CreatePickRequest, JoinLeague, UserLeaguesRequest},
        response_models::{
            CompetitorPick, CreateLeagueResponse, LeaderboardEntry, LeaderboardResponse,
            LeagueAthletesResponse, OpenLeagueResponse, ShotCallerPicksResponse,
            UserLeaguesPicksDataResponse, UserLeaguesPicksResponse, UserLeaguesResponse,
            WorkoutMovement, WorkoutResponse, WorkoutStage,
        },
    },
    repositories::league::LeagueRepository,
};
use log::info;
use sqlx::Error;
use std::collections::HashMap;
use tokio::join;

pub struct LeagueService;

impl LeagueService {
    pub async fn get_open_leagues(
        competition_id: &u64,
        user_id: &u64,
    ) -> Result<Vec<OpenLeagueResponse>, Error> {
        LeagueRepository::fetch_open_leagues(competition_id, user_id).await
    }

    pub async fn get_user_leagues(
        user_league: &UserLeaguesRequest,
    ) -> Result<Vec<UserLeaguesResponse>, Error> {
        LeagueRepository::fetch_user_leagues(user_league).await
    }

    async fn delete_pick(pick_request: &CreatePickRequest) -> Result<(), Error> {
        let current_picks = Self::get_user_league_picks(&pick_request.tournament_user_id).await?;

        let mut picks = current_picks.men_picks;
        picks.extend(current_picks.women_picks.clone());

        let deleted_pick = picks
            .iter()
            .find(|p| {
                p.competitor_id == (pick_request.previous_pick.competitor_id as u64)
                    && p.rank == (pick_request.previous_pick.rank as u64)
            })
            .unwrap();

        LeagueRepository::delete_user_league_pick(deleted_pick.id).await
    }

    pub async fn save_user_league_pick(pick_request: &CreatePickRequest) -> Result<(), Error> {
        let event_status =
            LeagueRepository::fetch_competition_tournament_status(pick_request.tournament_user_id)
                .await?;

        if (event_status.tournament_type_id == 1
            && (event_status.is_complete || event_status.is_complete))
            || (event_status.tournament_type_id == 2
                && pick_request.new_pick.rank <= event_status.locked_events)
        {
            return Err(Error::Protocol(
                "Can't update picks for an active or complete competition".to_string(),
            ));
        }

        if event_status.tournament_type_id == 1 {
            if pick_request.previous_pick.competitor_id != 0 {
                Self::delete_pick(pick_request).await?;
            }

            if pick_request.previous_pick.competitor_id == 0 {
                let user_picks =
                    LeagueRepository::fetch_user_league_picks(&pick_request.tournament_user_id)
                        .await?;

                let is_existing_pick = user_picks.iter().find(|up| {
                    up.rank == pick_request.new_pick.rank as u64
                        && up.competitor_id == pick_request.new_pick.competitor_id as u64
                });
                if is_existing_pick.is_none() {
                    LeagueRepository::insert_user_league_pick(
                        pick_request.tournament_user_id,
                        pick_request.new_pick.competitor_id,
                        pick_request.new_pick.rank,
                    )
                    .await?;
                } else {
                    return Err(Error::Protocol(
                        "Can't insert a pick that already exists".to_string(),
                    ));
                }
            }
        } else {
            if pick_request.previous_pick.competitor_id != 0 {
                Self::delete_pick(pick_request).await?;
            }
            if pick_request.new_pick.competitor_id != 0 {
                let user_picks =
                    LeagueRepository::fetch_user_league_picks(&pick_request.tournament_user_id)
                        .await?;

                let is_existing_pick = user_picks.iter().find(|up| {
                    up.rank == pick_request.new_pick.rank as u64
                        && up.competitor_id == pick_request.new_pick.competitor_id as u64
                });

                if is_existing_pick.is_none() {
                    LeagueRepository::insert_user_league_pick(
                        pick_request.tournament_user_id,
                        pick_request.new_pick.competitor_id,
                        pick_request.new_pick.rank,
                    )
                    .await?;
                } else {
                    return Err(Error::Protocol(
                        "Can't insert a pick that already exists".to_string(),
                    ));
                }
            }
        }

        return Ok(());
    }

    pub async fn get_league_leaderboard(tournament_id: &i64) -> Result<LeaderboardResponse, Error> {
        info!("get_league_leaderboard: {}", tournament_id);

        let metadata = LeagueRepository::fetch_competition(*tournament_id).await?;
        let tournament_users = LeagueRepository::fetch_tournament_users(*tournament_id).await?;
        let men_leaderboard =
            LeagueRepository::fetch_competition_leaderboard(metadata.competition_id as i64, 1)
                .await?;
        let women_leaderboard =
            LeagueRepository::fetch_competition_leaderboard(metadata.competition_id as i64, 2)
                .await?;

        let leaderboard = LeaderboardResponse {
            tournament: metadata.tournament_name,
            competition: metadata.competition_name,
            logo: metadata.competition_logo,
            locked_events: metadata.locked_events,
            leaderboard: if metadata.tournament_type_id == 1 {
                Self::get_top_10_entries(metadata.competition_id as i64, *tournament_id).await?
            } else {
                Self::get_top_shot_caller_entries(
                    tournament_users,
                    men_leaderboard,
                    women_leaderboard,
                    metadata.locked_events as i64,
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
            .map(|p: CompetitionLeaderboardResponse| {
                let mut points: f64 = 0.0;

                if tournament_type_id == 1 {
                    points = 10.0;
                } else {
                    points = 100.0;
                };
                MatchupPick {
                    predicted_rank: p.placement as u64,
                    rank: p.placement as u64,
                    first_name: p.first_name.clone(),
                    last_name: p.last_name.clone(),
                    competitor_id: p.competitor_id.clone() as u64,
                    points,
                    event_points: p.points,
                    is_withdrawn: false,
                    is_cut: false,
                    is_suspended: false,
                    is_final: false,
                }
            })
            .collect()
    }

    fn get_matchup_picks(
        tournament_type_id: i64,
        picks: Vec<LeaderboardPicks>,
        leaderboard: &HashMap<i64, CompetitionLeaderboardResponse>,
    ) -> Vec<MatchupPick> {
        picks
            .iter()
            .map(|p| {
                let competitor_leaderboard =
                    Self::get_competitor_leaderboard(&leaderboard, p.competitor_id);

                let mut points: f64 = 0.0;

                if tournament_type_id == 1 {
                    let current_rank = competitor_leaderboard.placement;
                    if current_rank != 0 && current_rank <= 15 {
                        let mut rank_diff = p.rank - current_rank;

                        if rank_diff < 0 {
                            rank_diff = rank_diff * -1;
                        }

                        points = (10 - rank_diff) as f64;

                        if points < 0.0 {
                            points = 0.0;
                        }
                    }
                } else {
                    points = competitor_leaderboard
                        .finishes
                        .get(p.rank as usize - 1)
                        .unwrap_or(&0.0)
                        .clone();
                };

                let withdrawn_ids = vec![&317272i64, &73659i64, &298928i64];
                let is_withdrawn = if competitor_leaderboard.competitor_id == 317272i64
                    || competitor_leaderboard.competitor_id == 73659
                    || competitor_leaderboard.competitor_id == 298928i64
                {
                    true
                } else {
                    false
                };
                MatchupPick {
                    predicted_rank: p.rank as u64,
                    rank: competitor_leaderboard.placement as u64,
                    first_name: competitor_leaderboard.first_name.clone(),
                    last_name: competitor_leaderboard.last_name.clone(),
                    competitor_id: competitor_leaderboard.competitor_id.clone() as u64,
                    points,
                    event_points: competitor_leaderboard.points,
                    is_withdrawn,
                    is_cut: false,
                    is_suspended: false,
                    is_final: is_withdrawn,
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

        // let tournament_users = LeagueRepository::fetch_tournament_users(tournament_id).await?;
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

        let user_men_players = Self::get_matchup_picks(
            metadata.tournament_type_id as i64,
            user_picks.men_competitor_ids.clone(),
            &men_leaderboard,
        );

        let user_women_players = Self::get_matchup_picks(
            metadata.tournament_type_id as i64,
            user_picks.women_competitor_ids.clone(),
            &women_leaderboard,
        );
        let competitor_men_players = if *competitor_id == 0 {
            Self::get_top_10_picks(metadata.tournament_type_id as i64, &men_leaderboard)
        } else {
            Self::get_matchup_picks(
                metadata.tournament_type_id as i64,
                competitor_picks.men_competitor_ids.clone(),
                &men_leaderboard,
            )
        };

        let competitor_women_players = if *competitor_id == 0 {
            Self::get_top_10_picks(metadata.tournament_type_id as i64, &women_leaderboard)
        } else {
            Self::get_matchup_picks(
                metadata.tournament_type_id as i64,
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

        // if metadata.tournament_type_id == 1 {
        //     let leaderboard_entries =
        //         Self::get_top_10_entries(metadata.competition_id, *tournament_id).await?;
        //
        //     leaderboard.leaderboard = leaderboard_entries;
        // } else {
        //     let leaderboard_entries =
        //         Self::get_top_shot_caller_entries(metadata.competition_id, *tournament_id).await?;
        //
        //     leaderboard.leaderboard = leaderboard_entries;
        // }

        Ok(leaderboard)
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
            })
            .clone()
    }

    async fn get_top_shot_caller_entries(
        tournament_users: Vec<LeaderboardTournamentUserData>,
        men_leaderboard: HashMap<i64, CompetitionLeaderboardResponse>,
        women_leaderboard: HashMap<i64, CompetitionLeaderboardResponse>,
        locked_events: i64,
    ) -> Result<Vec<LeaderboardEntry>, Error> {
        let mut leaderboard_entries: Vec<LeaderboardEntry> = tournament_users
            .iter()
            .map(|tu| {
                let men_points = tu
                    .men_competitor_ids
                    .iter()
                    .filter(|c| c.rank <= locked_events)
                    .map(|p| {
                        Self::get_competitor_leaderboard(&men_leaderboard, p.competitor_id)
                            .finishes
                            .get(p.rank as usize - 1)
                            .unwrap_or(&0.0)
                            .clone()
                    })
                    .sum();
                let women_points = tu
                    .women_competitor_ids
                    .iter()
                    .filter(|c| c.rank <= locked_events)
                    .map(|p| {
                        Self::get_competitor_leaderboard(&women_leaderboard, p.competitor_id)
                            .finishes
                            .get(p.rank as usize - 1)
                            .unwrap_or(&0.0)
                            .clone()
                    })
                    .sum();

                LeaderboardEntry {
                    tournament_user_id: tu.tournament_user_id,
                    display_name: tu.display_name.clone(),
                    avatar: tu.avatar.clone(),
                    men_points,
                    women_points,
                    points: men_points + women_points,
                }
            })
            .collect();

        leaderboard_entries.sort_by(|a, b| b.points.partial_cmp(&a.points).unwrap());

        Ok(leaderboard_entries)
    }

    async fn get_top_10_entries(
        competition_id: i64,
        tournament_id: i64,
    ) -> Result<Vec<LeaderboardEntry>, Error> {
        let tournament_users = LeagueRepository::fetch_tournament_users(tournament_id).await?;
        let men_leaderboard =
            LeagueRepository::fetch_competition_leaderboard(competition_id, 1).await?;
        let women_leaderboard =
            LeagueRepository::fetch_competition_leaderboard(competition_id, 2).await?;

        let mut leaderboard_entries: Vec<LeaderboardEntry> = tournament_users
            .iter()
            .map(|tu| {
                let men_points = tu
                    .men_competitor_ids
                    .iter()
                    .map(|p| {
                        let current_rank =
                            Self::get_competitor_leaderboard(&men_leaderboard, p.competitor_id)
                                .placement;

                        if current_rank == 0 || current_rank > 15 {
                            return 0.0;
                        }
                        let mut rank_diff = p.rank - current_rank;

                        if rank_diff < 0 {
                            rank_diff = rank_diff * -1;
                        }

                        // if current_rank == 0 || current_rank > 15 {
                        //     return 0.0;
                        // }

                        let points = (10 - rank_diff) as f64;

                        if points < 0.0 {
                            return 0.0;
                        }

                        return points;
                    })
                    .sum();

                let women_points = tu
                    .women_competitor_ids
                    .iter()
                    .map(|p| {
                        let current_rank =
                            Self::get_competitor_leaderboard(&women_leaderboard, p.competitor_id)
                                .placement;
                        if current_rank == 0 || current_rank > 15 {
                            return 0.0;
                        }
                        let mut rank_diff = p.rank - current_rank;

                        if rank_diff < 0 {
                            rank_diff = rank_diff * -1;
                        }

                        let points = (10 - rank_diff) as f64;

                        if points < 0.0 {
                            return 0.0;
                        }

                        return points;
                    })
                    .sum();
                LeaderboardEntry {
                    tournament_user_id: tu.tournament_user_id,
                    display_name: tu.display_name.clone(),
                    avatar: tu.avatar.clone(),
                    men_points,
                    women_points,
                    points: men_points + women_points,
                }
            })
            .collect();

        leaderboard_entries.sort_by(|a, b| b.points.partial_cmp(&a.points).unwrap());

        Ok(leaderboard_entries)
    }

    fn get_gender_picks(
        picks: &Vec<UserLeaguesPicksDataResponse>,
        gender_id: u64,
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
        let league_picks = LeagueRepository::fetch_user_league_picks(user_tournament_id).await?;

        let user_league_picks = UserLeaguesPicksResponse {
            tournament_user_id: *user_tournament_id as u64,
            men_picks: Self::get_gender_picks(&league_picks, 1),
            women_picks: Self::get_gender_picks(&league_picks, 2),
        };

        Ok(user_league_picks)
    }

    pub async fn get_shot_caller_picks(
        user_tournament_id: &i64,
    ) -> Result<ShotCallerPicksResponse, Error> {
        let competition_id =
            LeagueRepository::fetch_user_tournament_competition_id(user_tournament_id).await?;
        let league_picks = LeagueRepository::fetch_user_league_picks(user_tournament_id).await?;
        let athletes = LeagueRepository::fetch_league_athletes(competition_id as u64).await?;
        let competition_workouts = LeagueRepository::fetch_workouts(competition_id).await?;
        let workout_stages = LeagueRepository::fetch_workout_stages(competition_id).await?;
        let workout_stage_movements =
            LeagueRepository::fetch_workout_stage_movements(competition_id).await?;

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
                    name: w.name.clone(),
                    ordinal: w.ordinal,
                    start_time: w.start_time.clone(),
                    location: w.location.clone(),
                    description: w.description.clone(),
                    is_active: w.is_active,
                    is_complete: w.is_complete,
                    stages: Some(stages),
                }
            })
            .collect();

        let user_league_picks = ShotCallerPicksResponse {
            athletes,
            workouts,
            men_picks: Self::get_gender_picks(&league_picks, 1),
            women_picks: Self::get_gender_picks(&league_picks, 2),
        };

        Ok(user_league_picks)
    }

    pub async fn get_league_athletes(
        competition_id: &u64,
    ) -> Result<Vec<LeagueAthletesResponse>, Error> {
        LeagueRepository::fetch_league_athletes(*competition_id).await
    }
    pub async fn create_league(league: &CreateLeague) -> Result<CreateLeagueResponse, Error> {
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
        };

        let league_id = LeagueRepository::insert_tournament(new_league).await?;
        LeagueRepository::insert_tournament_user(league_id as i64, league.user_id as i64).await?;

        let created_league = CreateLeagueResponse {
            id: league_id,
            name: league.name.clone(),
            user_id: league.user_id,
            competition_id: league.competition_id,
            tournament_type_id: league.tournament_type_id,
            is_private: league.is_private,
            passcode: league.passcode.clone(),
        };

        Ok(created_league)
    }

    pub async fn update_scores(scores: &InsertScoresRequest) -> Result<(), Error> {
        let mut current_scores =
            LeagueRepository::fetch_scores(scores.competition_id, scores.ordinal).await?;

        for s in scores.scores.clone() {
            let mut existing_score = current_scores
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
                pick_sum as f64 + (tournament_entries as f64 - v.len() as f64) * 11.0
            } else {
                pick_sum as f64
            };

            let adp = competitor_pick_sum / tournament_entries as f64;
            LeagueRepository::update_competitor_adp(*k, adp).await?;
        }

        Ok(())
    }

    pub async fn update_adp() -> Result<(), Error> {
        join!(
            Self::update_competition_gender_adp(13, 1),
            Self::update_competition_gender_adp(13, 2),
            Self::update_competition_gender_adp(14, 1),
            Self::update_competition_gender_adp(14, 2),
            Self::update_competition_gender_adp(17, 1),
            Self::update_competition_gender_adp(17, 2)
        );

        Ok(())
    }
}
