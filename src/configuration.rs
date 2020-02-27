use serde_derive::Deserialize;
use std::fs;

#[derive(Deserialize)]
pub struct Config {
    pub ip: String,
    pub debug: bool,
    pub handshake: Option<u16>,
    pub redis: String,
}

pub fn parse_config() -> Config {
    let conffile = fs::read_to_string("/etc/aegis.toml");
    let config: Config = toml::from_str(conffile).unwrap();
    return config;
}
