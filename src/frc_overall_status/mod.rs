use Api;
use serde_json::Value;
use std::collections::HashMap;

pub trait OverallStatus {
    fn status(self) -> Value;
}

impl OverallStatus for Api {
    fn status(self) -> Value {
        let endpoint = "/status";
        self.call_endpoint(endpoint, HashMap::new())
    }
}
