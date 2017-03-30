use Api;
use ModelType;
use serde_json::Value;
use std::collections::HashMap;

pub trait Team {
    fn team(&mut self, team_key: &str, model_type: Option<ModelType>) -> Value;
    fn team_awards(&mut self, team_key: &str, year: Option<u32>) -> Value;
    fn team_districts(&mut self, team_key: &str) -> Value;
    fn team_event_awards(&mut self, team_key: &str, event_key: &str) -> Value;
    fn team_event_matches(&mut self,
                          team_key: &str,
                          event_key: &str,
                          model_type: Option<ModelType>)
                          -> Value;
    fn team_event_status(&mut self, team_key: &str, event_key: &str) -> Value;
    fn team_events(&mut self,
                   team_key: &str,
                   year: Option<u32>,
                   model_type: Option<ModelType>)
                   -> Value;
    fn team_matches(&mut self, team_key: &str, year: u32, model_type: Option<ModelType>) -> Value;
    fn team_media(&mut self, team_key: &str, year: u32) -> Value;
    fn team_robots(&mut self, team_key: &str) -> Value;
    fn team_social_media(&mut self, team_key: &str) -> Value;
    fn team_years_participated(&mut self, team_key: &str) -> Value;
}

impl Team for Api {
    fn team(&mut self, team_key: &str, model_type: Option<ModelType>) -> Value {
        let endpoint = "/team/<team_key>/<model_type>";

        let mut args = HashMap::new();
        args.insert(String::from("team_key"), String::from(team_key));

        let _ = match model_type {
            Some(mt) => {
                match mt {
                    ModelType::Simple => {
                        args.insert(String::from("model_type"), String::from("simple"))
                    }
                    ModelType::Keys => panic!("Model Type cannot be 'Keys' for Team trait"),
                }
            }
            None => None,
        };

        self.call_endpoint(endpoint, args)
    }

    fn team_awards(&mut self, team_key: &str, year: Option<u32>) -> Value {
        let endpoint = "/team/<team_key>/awards/<year>";

        let mut args = HashMap::new();
        args.insert(String::from("team_key"), String::from(team_key));

        let _ = match year {
            Some(y) => args.insert(String::from("year"), y.to_string()),
            None => None,
        };


        self.call_endpoint(endpoint, args)
    }

    fn team_event_matches(&mut self,
                          team_key: &str,
                          event_key: &str,
                          model_type: Option<ModelType>)
                          -> Value {
        let endpoint = "/team/<team_key>/event/<event_key>/matches/<model_type>";

        let mut args = HashMap::new();
        args.insert(String::from("team_key"), String::from(team_key));
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

    fn team_event_awards(&mut self, team_key: &str, event_key: &str) -> Value {
        let endpoint = "/team/<team_key>/event/<event_key>/awards";

        let mut args = HashMap::new();
        args.insert(String::from("team_key"), String::from(team_key));
        args.insert(String::from("event_key"), String::from(event_key));


        self.call_endpoint(endpoint, args)
    }

    fn team_event_status(&mut self, team_key: &str, event_key: &str) -> Value {
        let endpoint = "/team/<team_key>/event/<event_key>/status";

        let mut args = HashMap::new();
        args.insert(String::from("team_key"), String::from(team_key));
        args.insert(String::from("event_key"), String::from(event_key));


        self.call_endpoint(endpoint, args)
    }

    fn team_events(&mut self,
                   team_key: &str,
                   year: Option<u32>,
                   model_type: Option<ModelType>)
                   -> Value {
        let endpoint = "/team/<team_key>/events/<year>/<model_type>";

        let mut args = HashMap::new();
        args.insert(String::from("team_key"), String::from(team_key));

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


    fn team_matches(&mut self, team_key: &str, year: u32, model_type: Option<ModelType>) -> Value {
        let endpoint = "/team/<team_key>/matches/<year>/<model_type>";

        let mut args = HashMap::new();
        args.insert(String::from("team_key"), String::from(team_key));
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

    fn team_media(&mut self, team_key: &str, year: u32) -> Value {
        let endpoint = "/team/<team_key>/media/<year>";


        let mut args = HashMap::new();
        args.insert(String::from("team_key"), String::from(team_key));
        args.insert(String::from("year"), year.to_string());

        self.call_endpoint(endpoint, args)
    }

    fn team_years_participated(&mut self, team_key: &str) -> Value {
        let endpoint = "/team/<team_key>/years_participated";


        let mut args = HashMap::new();
        args.insert(String::from("team_key"), String::from(team_key));


        self.call_endpoint(endpoint, args)
    }

    fn team_districts(&mut self, team_key: &str) -> Value {
        let endpoint = "/team/<team_key>/districts";


        let mut args = HashMap::new();
        args.insert(String::from("team_key"), String::from(team_key));


        self.call_endpoint(endpoint, args)
    }

    fn team_robots(&mut self, team_key: &str) -> Value {
        let endpoint = "/team/<team_key>/robots";


        let mut args = HashMap::new();
        args.insert(String::from("team_key"), String::from(team_key));


        self.call_endpoint(endpoint, args)
    }

    fn team_social_media(&mut self, team_key: &str) -> Value {
        let endpoint = "/team/<team_key>/social_media";


        let mut args = HashMap::new();
        args.insert(String::from("team_key"), String::from(team_key));


        self.call_endpoint(endpoint, args)
    }
}
