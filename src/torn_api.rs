use crate::config::Config;
use reqwest::blocking::Client;
use reqwest::header::ACCEPT;
use reqwest::Url;
use serde_json::Value;
use user::User;

pub mod user;

pub struct ApiClient {
    base_url: String,
    config: Config,
    client: Client,
}

impl ApiClient {
    pub fn new(config: Config) -> ApiClient {
        return ApiClient {
            base_url: String::from("https://api.torn.com/v2"),
            config: config,
            client: Client::new(),
        };
    }
    fn request<T>(&self, url: Url) -> Value {
        let request = self.client.get(url).header(ACCEPT, "application/json");

        let response = request.send();
        let body = response
            .expect("No response from API")
            .text()
            .expect("API response not a string");

        return serde_json::from_str(&body).unwrap();
    }

    pub fn get_user(&self) -> Value {
        let url = self.base_url.clone();
        let url = format!("{url}/user");
        let params = [("key", self.config.api_key.clone())];
        let url = Url::parse_with_params(&url, &params).expect("Failed to build URL");

        return self.request::<User>(url);
    }
}
