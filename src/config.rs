use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub general: GeneralConfig,
    pub input: InputConfig,
    pub actions: ActionsConfig,
    pub pid: PidConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct GeneralConfig {
    pub mode: String,
    pub update_rate_hz: u32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct InputConfig {
    pub r#type: String,
    pub arduino_port: String,
    pub arduino_baud: u32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ActionsConfig {
    pub r#type: String,
    pub max_motor_speed: u8,
    pub encoder_ticks_per_revolution: u32,
    pub wheel_circumference_mm: f32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct PidConfig {
    pub kp: f32,
    pub ki: f32,
    pub kd: f32,
}

impl Config {
    pub fn from_file(path: &str) -> Result<Self, anyhow::Error> {
        let config_content = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&config_content)?;
        Ok(config)
    }
}
