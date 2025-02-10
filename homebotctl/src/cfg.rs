use serde::Deserialize;
use std::fs;
use std::path::Path;
use serde_yaml;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Config {
    pub code_path: String,
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
}

impl Config {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let config: Config = serde_yaml::from_str(&content)?;
        Ok(config)
    }
}

