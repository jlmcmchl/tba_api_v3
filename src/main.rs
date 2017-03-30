extern crate tba_api_v3;
use tba_api_v3::Api;
use tba_api_v3::frc_overall_status::OverallStatus;
use tba_api_v3::frc_teams::Teams;
use tba_api_v3::frc_team::Team;
use tba_api_v3::ModelType;


fn main() {
    let auth_key = "YOUR KEY HERE";

    let api = Api::new(auth_key);


    let res = api.clone().status();
    println!("{}", res);


    let res = api.clone().status();
    println!("{}", res);
}
