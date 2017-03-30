use Api;
use ModelType;
use serde_json::Value;
use std::collections::HashMap;

pub trait Events {
    fn events(&mut self, year: u32, model_type: Option<ModelType>) -> Value;
}

impl Events for Api {
    fn events(&mut self, year: u32, model_type: Option<ModelType>) -> Value {
        let endpoint = "/events/<year>/<model_type>";


        let mut args = HashMap::new();
        args.insert(String::from("year"), year.to_string());

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
