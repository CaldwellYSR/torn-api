use torn_api::config::Config;

fn main() {
    let config = Config::new();
    println!("TORN_API_KEY env var = {0:?}", config.api_key);
}
