//! Example showing how to use the Webots simulation backend
use homebot::{Config, inputs, actions, logic};

fn main() -> Result<(), anyhow::Error> {
    let config = Config::from_file("config.toml")?;
    
    // These will create Webots backends when mode is "webots"
    let mut input_source = inputs::create_input_source(&config)?;
    let mut action_target = actions::create_action_target(&config)?;
    
    let mut motor_control = logic::MotorControl::new(config);
    
    // Simple test: drive in a square pattern
    for _ in 0..4 {
        // Drive forward
        for _ in 0..10 {
            let sensors = input_source.read_sensors()?;
            motor_control.update_encoders(sensors.left_encoder, sensors.right_encoder);
            
            let commands = motor_control.compute_motor_commands(0.7, 0.0);
            action_target.set_motor_speeds(commands.left_speed, commands.right_speed)?;
            
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
        
        // Turn right
        for _ in 0..5 {
            let sensors = input_source.read_sensors()?;
            motor_control.update_encoders(sensors.left_encoder, sensors.right_encoder);
            
            let commands = motor_control.compute_motor_commands(0.3, 0.5);
            action_target.set_motor_speeds(commands.left_speed, commands.right_speed)?;
            
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    }
    
    action_target.stop_motors()?;
    Ok(())
}
