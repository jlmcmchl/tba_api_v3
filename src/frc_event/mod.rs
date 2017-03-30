use Api;
use ModelType;
use DetailType;
use serde_json::Value;
use std::collections::HashMap;

pub trait Event {
    fn event(&mut self, event_key: &str, model_type: Option<ModelType>) -> Value;
    fn event_detail(&mut self, event_key: &str, detail_type: DetailType) -> Value;
    fn event_awards(&mut self, event_key: &str) -> Value;
    fn event_matches(&mut self, event_key: &str, model_type: Option<ModelType>) -> Value;
    fn event_teams(&mut self, event_key: &str, model_type: Option<ModelType>) -> Value;
}

impl Event for Api {
    fn event(&mut self, event_key: &str, model_type: Option<ModelType>) -> Value {
        let endpoint = "/event/<event_key>/<model_type>";


        let mut args = HashMap::new();
        args.insert(String::from("event_key"), String::from(event_key));

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

    fn event_detail(&mut self, event_key: &str, detail_type: DetailType) -> Value {
        let endpoint = "/event/<event_key>/<detail_type>";


        let mut args = HashMap::new();
        args.insert(String::from("event_key"), String::from(event_key));

        let _ = match detail_type {
            DetailType::Alliances => {
                args.insert(String::from("detail_type"), String::from("alliances"))
            }
            DetailType::DistrictPoints => {
                args.insert(String::from("detail_type"), String::from("district_points"))
            }
            DetailType::Insights => {
                args.insert(String::from("detail_type"), String::from("insights"))
            }
            DetailType::Oprs => args.insert(String::from("detail_type"), String::from("oprs")),
            DetailType::Rankings => {
                args.insert(String::from("detail_type"), String::from("rankings"))
            }
        };


        self.call_endpoint(endpoint, args)
    }

    fn event_awards(&mut self, event_key: &str) -> Value {
        let endpoint = "/event/<event_key>/awards";


        let mut args = HashMap::new();
        args.insert(String::from("event_key"), String::from(event_key));


        self.call_endpoint(endpoint, args)
    }

    fn event_matches(&mut self, event_key: &str, model_type: Option<ModelType>) -> Value {
        let endpoint = "/event/<event_key>/matches/<model_type>";


        let mut args = HashMap::new();
        args.insert(String::from("event_key"), String::from(event_key));

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

    fn event_teams(&mut self, event_key: &str, model_type: Option<ModelType>) -> Value {
        let endpoint = "/event/<event_key>/teams/<model_type>";


        let mut args = HashMap::new();
        args.insert(String::from("event_key"), String::from(event_key));

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
