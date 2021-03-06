use Api;
use ModelType;
use serde_json::Value;
use std::collections::HashMap;

pub trait Match {
    fn match_detail(&mut self, match_key: &str, model_type: Option<ModelType>) -> Value;
}

impl Match for Api {
    fn match_detail(&mut self, match_key: &str, model_type: Option<ModelType>) -> Value {
        let endpoint = "/match/<match_key>/<model_type>";


        let mut args = HashMap::new();
        args.insert(String::from("match_key"), String::from(match_key));

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
