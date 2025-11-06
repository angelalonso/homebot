mod input_source;
mod webots_input;
mod arduino_input;

pub use input_source::InputSource;
pub use webots_input::WebotsInput;
pub use arduino_input::ArduinoInput;

use crate::config::Config;

/// Create input source based on configuration
pub fn create_input_source(config: &Config) -> Result<Box<dyn InputSource>, anyhow::Error> {
    match config.input.r#type.as_str() {
        "webots" => Ok(Box::new(WebotsInput::new()?)),
        "arduino" => Ok(Box::new(ArduinoInput::new(
            &config.input.arduino_port,
            config.input.arduino_baud,
        )?)),
        _ => Err(anyhow::anyhow!("Unknown input type: {}", config.input.r#type)),
    }
}
