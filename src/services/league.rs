use crate::handlers::league::response_models::WorkoutResponse;
use crate::{
    data::models::tournament::Tournament,
    handlers::league::{
        request_models::{CreateLeague, JoinLeague, UserLeaguesRequest},
        response_models::{
            CompetitorPick, CreateLeagueResponse, LeaderboardEntry, LeaderboardResponse,
            LeaderboardScores, LeaderboardShotCallerScoreData, LeaderboardTop10ScoreData,
            LeagueAthletesResponse, OpenLeagueResponse, ShotCallerPicksResponse,
            UserLeaguesPicksDataResponse, UserLeaguesPicksResponse, UserLeaguesResponse,
        },
    },
    repositories::league::LeagueRepository,
};
use log::info;
use sqlx::Error;
use std::collections::HashMap;

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

    pub async fn get_league_leaderboard(tournament_id: &i64) -> Result<LeaderboardResponse, Error> {
        info!("get_league_leaderboard: {}", tournament_id);

        let metadata = LeagueRepository::fetch_competition(*tournament_id).await?;

        let mut leaderboard = LeaderboardResponse {
            tournament: metadata.tournament_name,
            competition: metadata.competition_name,
            logo: metadata.competition_logo,
            locked_events: metadata.locked_events,
            leaderboard: vec![],
        };

        if metadata.tournament_type_id == 1 {
            let leaderboard_entries =
                Self::get_top_10_entries(metadata.competition_id, *tournament_id).await?;

            leaderboard.leaderboard = leaderboard_entries;
        } else {
            let leaderboard_entries =
                Self::get_top_shot_caller_entries(metadata.competition_id, *tournament_id).await?;

            leaderboard.leaderboard = leaderboard_entries;
        }

        Ok(leaderboard)
    }

    async fn get_ordered_top_10_scores(
        competition_id: u64,
        gender_id: i64,
    ) -> Result<HashMap<u64, LeaderboardTop10ScoreData>, Error> {
        let mut scores = LeagueRepository::fetch_top_10_scores(competition_id, gender_id).await?;

        scores.sort_by(|a, b| b.points.partial_cmp(&a.points).unwrap());

        scores
            .iter_mut()
            .enumerate()
            .for_each(|(i, s)| s.rank = (i as i64) + 1);

        Ok(scores.into_iter().map(|s| (s.competitor_id, s)).collect())
    }

    async fn get_top_shot_caller_entries(
        competition_id: u64,
        tournament_id: i64,
    ) -> Result<Vec<LeaderboardEntry>, Error> {
        let tournament_users = LeagueRepository::fetch_tournament_users(tournament_id).await?;

        let scores = LeagueRepository::fetch_shot_caller_scores(competition_id).await?;

        let mut leaderboard_entries: Vec<LeaderboardEntry> = tournament_users
            .iter()
            .map(|tu| {
                let men_points = tu
                    .men_competitor_ids
                    .iter()
                    // .filter(|c| c.rank <= locked_events as i64)
                    .map(|p| {
                        scores
                            .iter()
                            .find(|s| s.competitor_id == p.competitor_id as u64)
                            .unwrap_or(&LeaderboardShotCallerScoreData {
                                competitor_id: 0,
                                men_competitors: vec![],
                                women_competitors: vec![],
                            })
                            .men_competitors
                            .iter()
                            .find(|s| s.ordinal == p.rank)
                            .unwrap_or(&LeaderboardScores {
                                ordinal: 0,
                                points: 0.0,
                            })
                            .points
                    })
                    .sum();
                let women_points = tu
                    .men_competitor_ids
                    .iter()
                    // .filter(|c| c.rank <= locked_events as i64)
                    .map(|p| {
                        scores
                            .iter()
                            .find(|s| s.competitor_id == p.competitor_id as u64)
                            .unwrap_or(&LeaderboardShotCallerScoreData {
                                competitor_id: 0,
                                men_competitors: vec![],
                                women_competitors: vec![],
                            })
                            .women_competitors
                            .iter()
                            .find(|s| s.ordinal == p.rank)
                            .unwrap_or(&LeaderboardScores {
                                ordinal: 0,
                                points: 0.0,
                            })
                            .points
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
        competition_id: u64,
        tournament_id: i64,
    ) -> Result<Vec<LeaderboardEntry>, Error> {
        let tournament_users = LeagueRepository::fetch_tournament_users(tournament_id).await?;

        let men_scores = Self::get_ordered_top_10_scores(competition_id, 1).await?;
        let women_scores = Self::get_ordered_top_10_scores(competition_id, 2).await?;

        let mut leaderboard_entries: Vec<LeaderboardEntry> = tournament_users
            .iter()
            .map(|tu| {
                let men_points = tu
                    .men_competitor_ids
                    .iter()
                    .map(|p| {
                        let current_rank = men_scores
                            .get(&(p.competitor_id as u64))
                            .unwrap_or(&LeaderboardTop10ScoreData {
                                points: None,
                                rank: 0,
                                competitor_id: 0,
                            })
                            .rank;
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

                let women_points = tu
                    .women_competitor_ids
                    .iter()
                    .map(|p| {
                        let current_rank = women_scores
                            .get(&(p.competitor_id as u64))
                            .unwrap_or(&LeaderboardTop10ScoreData {
                                points: None,
                                rank: 0,
                                competitor_id: 0,
                            })
                            .rank;
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

        let workouts = competition_workouts
            .iter()
            .map(|w| WorkoutResponse {
                name: w.name.clone(),
                ordinal: w.ordinal,
                start_time: w.start_time.clone(),
                location: w.location.clone(),
                description: w.description.clone(),
                is_active: w.is_active,
                is_complete: w.is_complete,
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
}
