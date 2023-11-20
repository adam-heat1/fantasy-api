use super::region::Region;
use serde::Serialize;

#[derive(Serialize)]
pub struct Competition {
    pub id: u64,
    pub name: String,
    pub is_active: bool,
    pub is_complete: bool,
    pub logo: String,
    pub region_id: u64,
    pub sort_order: u64,

    pub region: Option<Region>,
}