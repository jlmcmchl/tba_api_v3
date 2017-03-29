#![crate_type = "lib"]
#![crate_name = "tba_api_v3"]
#![feature(use_extern_macros)]

#[macro_use] extern crate hyper;
extern crate serde_json;
extern crate hyper_native_tls;
extern crate regex;

use hyper::client::Client;
use std::collections::HashMap;

pub mod frc_district;
pub mod frc_district_list;
pub mod frc_event;
pub mod frc_event_list;
pub mod frc_match;
pub mod frc_overall_status;
pub mod frc_team;
pub mod frc_team_awards;
pub mod frc_team_event;
pub mod frc_team_events;
pub mod frc_team_history;
pub mod frc_team_list;
pub mod api;

header! { (XTBAAuthKey, "X-TBA-Auth-Key") => [String] }

pub enum ModelType {
    Simple,
    Keys
}

pub enum DetailType {
    Alliances,
    DistrictPoints,
    Insights,
    Oprs,
    Rankings
}

pub struct Api {
    auth_key: String,
    client: Client,
    args: HashMap<String, String>
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
