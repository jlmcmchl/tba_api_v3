use Api;
use serde_json::Value;
use std::collections::HashMap;

pub trait Districts {
    fn districts(&mut self, year: u32) -> Value;
}

impl Districts for Api {
    fn districts(&mut self, year: u32) -> Value {
        let endpoint = "/districts/<year>";

        let mut args = HashMap::new();
        args.insert(String::from("year"), year.to_string());

        self.call_endpoint(endpoint, args)
    }
}
