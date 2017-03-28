#![crate_type = "lib"]
#![crate_name = "tba_api_v3"]

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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
