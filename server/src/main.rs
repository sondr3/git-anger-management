use dotenv::dotenv;
use serde::Deserialize;
use std::env;
use warp::Filter;

#[derive(Debug, Deserialize, Clone)]
#[serde(default)]
struct Config {
    admin: String,
    port: usize,
    domain: String,
    path: String,
    name: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            admin: "".to_owned(),
            port: 8080,
            domain: "localhost".to_owned(),
            path: "/".to_owned(),
            name: "anger".to_owned(),
        }
    }
}

#[tokio::main]
async fn main() {
    std::env::set_var("RUST_LOG", "anger=info");
    pretty_env_logger::init();
    dotenv().ok();

    let config: Config = match envy::prefixed("ANGER_").from_env() {
        Ok(config) => config,
        Err(err) => panic!(err),
    };

    let host = format!("{}:{}", config.domain, config.port);

    println!("Server running on: http://{}", host);
}
