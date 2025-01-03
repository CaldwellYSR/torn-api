use std::env;

pub struct Config {
    pub api_key: String,
}

impl Config {
    pub fn new() -> Config {
        let api_key = match env::var("TORN_API_KEY") {
            Ok(val) => val,
            Err(_) => String::new(),
        };

        return Config { api_key: api_key };
    }
}
