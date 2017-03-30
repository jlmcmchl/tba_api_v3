use Api;
use serde_json::Value;
use ModelType;
use std::collections::HashMap;

pub trait District {
    fn district_events(&mut self, district_key: &str, model_type: Option<ModelType>) -> Value;
    fn district_rankings(&mut self, district_key: &str) -> Value;
    fn district_teams(&mut self, district_key: &str, model_type: Option<ModelType>) -> Value;
}

impl District for Api {
    fn district_events(&mut self, district_key: &str, model_type: Option<ModelType>) -> Value {
        let endpoint = "/district/<district_key>/events/<model_type>";

        let mut args = HashMap::new();
        args.insert(String::from("district_key"), String::from(district_key));

        let _ = match model_type {
            Some(mt) => {
                match mt {
                    ModelType::Simple => {
                        args.insert(String::from("model_type"), String::from("simple"))
                    }
                    ModelType::Keys => {
                        args.insert(String::from("model_type"), String::from("keys"))
                    }
                }
            }
            None => None,
        };

        self.call_endpoint(endpoint, args)
    }

    fn district_rankings(&mut self, district_key: &str) -> Value {
        let endpoint = "/district/<district_key>/rankings";

        let mut args = HashMap::new();
        args.insert(String::from("district_key"), String::from(district_key));

        self.call_endpoint(endpoint, args)
    }

    fn district_teams(&mut self, district_key: &str, model_type: Option<ModelType>) -> Value {
        let endpoint = "/district/<district_key>/teams/<model_type>";

        let mut args = HashMap::new();
        args.insert(String::from("district_key"), String::from(district_key));

        let _ = match model_type {
            Some(mt) => {
                match mt {
                    ModelType::Simple => {
                        args.insert(String::from("model_type"), String::from("simple"))
                    }
                    ModelType::Keys => {
                        args.insert(String::from("model_type"), String::from("keys"))
                    }
                }
            }
            None => None,
        };

        self.call_endpoint(endpoint, args)
    }
}
