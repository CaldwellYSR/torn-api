use config::Config;

pub mod config;

fn main() {
    let config = Config::new();
    println!("TORN_API_KEY env var = {0:?}", config.api_key);
}
