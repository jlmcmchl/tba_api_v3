use Api;
use serde_json::Value;

pub trait OverallStatus {
    fn status(self) -> Value;
}

impl OverallStatus for Api {
    fn status(self) -> Value {
        let endpoint = "/status";
        self.call_endpoint(endpoint)
    }
}