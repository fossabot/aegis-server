use serde_derive::Deserialize;
use std::fs;

#[derive(Deserialize)]
pub struct Config {
    ip: String,
    debug: bool,
    handshake: Option<u16>,
    redis: String,
}

pub fn parse_config() -> Config {
    let conffile = fs::read_to_string("/etc/aegis.toml");
    let config: Config = toml::from_str(conffile).unwrap();
    return config;
}
