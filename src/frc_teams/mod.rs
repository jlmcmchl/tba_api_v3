use Api;
use ModelType;
use serde_json::Value;
use std::collections::HashMap;

pub trait Teams {
    fn teams(&mut self, page_num: u32, year: Option<u32>, model_type: Option<ModelType>) -> Value;
}

impl Teams for Api {
    fn teams(&mut self, page_num: u32, year: Option<u32>, model_type: Option<ModelType>) -> Value {
        let endpoint = "/teams/<year>/<page_num>/<model_type>";

        let mut args = HashMap::new();
        args.insert(String::from("page_num"), page_num.to_string());

        let _ = match year {
            Some(y) => args.insert(String::from("year"), y.to_string()),
            None => None,
        };

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
