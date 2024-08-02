pub struct CrossfitRepository;

impl CrossfitRepository {
    // pub async fn fetch_leaderboard(
    //     leaderboard_type: String,
    //     year: i64,
    //     division: i64,
    //     region: i64,
    //     page: i64,
    //     ordinal: i64,
    // ) -> Result<CrossfitLeaderboard, Error> {
    //     let url = format!(
    //         "https://c3po.crossfit.com/api/competitions/v2/competitions/{}/{}/leaderboards?division={}&region={}&page={}&sort={}",
    //         leaderboard_type, year, division, region, page, ordinal
    //     );
    //
    //     let response = Client::new().get(url).send().await?;
    //
    //     // if response.status() == StatusCode::OK {
    //     //     let leaderboard = response.json::<CrossfitLeaderboard>().await?;
    //     //     Ok(leaderboard)
    //     // }
    //     //
    //     // Err(Error())
    //
    //     // response.json::<CrossfitLeaderboard>().await.unwrap();
    //
    //     match response.status() {
    //         StatusCode::OK => {
    //             // on success, parse our JSON to an APIResponse
    //             match response.json::<CrossfitLeaderboard>().await {
    //                 Ok(leaderboard) => return Ok(leaderboard),
    //                 Err(_) => return Err("Hm, the response didn't match the shape we expected."),
    //             };
    //         }
    //         StatusCode::UNAUTHORIZED => {
    //             return Err("Need to grab a new token");
    //         }
    //         other => {
    //             return Err(format!("Uh oh! Something unexpected happened: {:?}", other));
    //         }
    //     }?
    // }
}
