use std::collections::HashMap;
use std::io::Read;
use std::sync::Arc;
use hyper::client::Client;
use hyper::net::HttpsConnector;
use hyper::Url;
use hyper_native_tls::NativeTlsClient;
use serde_json;
use serde_json::Value;
use regex::Regex;
use Api;
use XTBAAuthKey;

const BASE: &str = "https://www.thebluealliance.com/api/v3";

fn fill_address(fmt: &str, args: HashMap<String, String>) -> Url {
    let unused_args = Regex::new(r"/<[^>]+>").unwrap();

    let mut endpoint = String::from(fmt);

    for (key, value) in args.iter() {
        endpoint = endpoint.replace(format!("<{}>", key).as_str(), value.as_str());
    }

    let mut address = String::from(BASE);
    address.push_str(endpoint.as_str());

    let url = unused_args.replace_all(&address, "");

    Url::parse(&url).unwrap()
}

impl Api {
    pub fn call_endpoint(&self, fmt: &str, args: HashMap<String, String>) -> Value {
        let url = fill_address(fmt, args);
        println!("{}", url);

        let ret = self.client
            .get(url)
            .header(XTBAAuthKey(self.auth_key.clone()))
            .send();
        match ret {
            Ok(mut result) => {
                let mut response = String::new();

                match result.read_to_string(&mut response) {
                    Ok(i) => {
                        println!("Read {} bytes", i);
                        match serde_json::from_str(response.as_str()) {
                            Ok(v) => v,
                            Err(e) => {
                                println!("{}", response);
                                panic!(e)
                            }
                        }
                    }
                    Err(e) => panic!(e),
                }
            }
            Err(e) => panic!(e),
        }
    }

    pub fn new(auth_key: &str) -> Api {
        let ssl = NativeTlsClient::new().unwrap();
        let connector = HttpsConnector::new(ssl);

        Api {
            auth_key: String::from(auth_key),
            client: Arc::new(Client::with_connector(connector)),
        }
    }
}
