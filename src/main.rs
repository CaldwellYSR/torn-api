use clap::Parser;
use config::Config;
use reqwest::blocking::Client;
use reqwest::header::ACCEPT;
use reqwest::Url;
use serde_json::Value;

pub mod config;

#[derive(Parser)]
struct Args {
    command: String,
}

fn main() {
    let config = Config::new();

    let args = Args::parse();

    println!("TORN_API_KEY env var = {0:?}", config.api_key);

    if args.command == "user" {
        let json = self::call_user_endpoint(config);
        println!("{:#?}", json);
    }
}

fn call_user_endpoint(config: Config) -> Value {
    let url = "https://api.torn.com/v2/user";
    let params = [("key", config.api_key)];
    let url = Url::parse_with_params(url, &params).expect("Failed to build URL");

    let client = Client::new();
    let request = client.get(url).header(ACCEPT, "application/json");

    let response = request.send();
    let body = response
        .expect("No response from API")
        .text()
        .expect("API response not a string");

    let json: Value = serde_json::from_str(&body).expect("Poorly formed JSON");

    return json;
}
