mod action_target;
mod webots_actions;
mod arduino_actions;

pub use action_target::ActionTarget;
pub use webots_actions::WebotsActions;
pub use arduino_actions::ArduinoActions;

use crate::config::Config;

/// Create action target based on configuration
pub fn create_action_target(config: &Config) -> Result<Box<dyn ActionTarget>, anyhow::Error> {
    match config.actions.r#type.as_str() {
        "webots" => Ok(Box::new(WebotsActions::new()?)),
        "arduino" => Ok(Box::new(ArduinoActions::new(
            &config.input.arduino_port,
            config.input.arduino_baud,
        )?)),
        _ => Err(anyhow::anyhow!("Unknown action type: {}", config.actions.r#type)),
    }
}
