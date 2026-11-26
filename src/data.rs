use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub addr1: Ip1,
    pub port: Port1,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Ip1 {
    pub ip: String,
    pub port: u16,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Port1 {
    pub timeout: u64,
}

pub fn load(path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let config: Config = serde_yaml::from_str(&content)?;
    Ok(config)
}

pub fn save(path: &str, cfg: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let yaml = serde_yaml::to_string(cfg)?;
    fs::write(path, yaml)?;
    Ok(())
}
