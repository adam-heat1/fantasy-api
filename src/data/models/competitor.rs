use super::{competitor_status::CompetitorStatus, country::Country, division::Division, gender::Gender, region::Region};
use serde::Serialize;

#[derive(Serialize)]
pub struct Competitor {
    pub id: u64,
    pub gender_id: u64,
    pub country_id: u64,
    pub region_id: u64,
    pub status_id: u64,
    pub division_id: u64,
    pub first_name: String,
    pub last_name: String,
    pub age: u64,
    pub height: u64,
    pub weight: u64,
    pub profile_url: u64,
    pub crossfit_id: u64,
    pub instagram: String,

    pub status: CompetitorStatus,
    pub country: Country,
    pub gender: Gender,
    pub region: Region,
    pub division: Division,
}