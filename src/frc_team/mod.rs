use Api;
use ModelType;
use serde_json::Value;

pub trait Team{
    fn team(&mut self, team_key: &str, model_type: Option<ModelType>) -> Value;
}

impl Team for Api {
    fn team(&mut self, team_key: &str, model_type: Option<ModelType>) -> Value {
        let endpoint = "/team/<team_key>/<model_type>";


        self.args.insert(String::from("team_key"), String::from(team_key));

        let _ = match model_type {
            Some(mt) => match mt {
                ModelType::Simple => self.args.insert(String::from("model_type"), String::from("simple")),
                ModelType::Keys => panic!("Model Type cannot be 'Keys' for Team trait")
            },
            None => None
        };

        self.call_endpoint(endpoint)
    }
}