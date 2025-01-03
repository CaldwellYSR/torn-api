use clap::Parser;
use config::Config;
use serde_json::Value;
use torn_api::ApiClient;

pub mod config;
pub mod torn_api;

#[derive(Parser)]
struct Args {
    command: String,
}

fn main() {
    let config = Config::new();

    let args = Args::parse();

    if args.command == "user" {
        let json = self::call_user_endpoint(config);
        println!("{:#?}", json);
    }
}

fn call_user_endpoint(config: Config) -> Value {
    let client = ApiClient::new(config);
    return client.get_user();
}
