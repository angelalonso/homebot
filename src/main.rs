use homebot::{Config, inputs, actions, logic};
use std::thread;
use std::time::{Duration, Instant};

fn main() -> Result<(), anyhow::Error> {
    // Load configuration
    let config = Config::from_file("config.toml")?;
    
    // Initialize input source and action target based on config
    let mut input_source = inputs::create_input_source(&config)?;
    let mut action_target = actions::create_action_target(&config)?;
    
    // Initialize motor control logic
    let mut motor_control = logic::MotorControl::new(config.clone());
    
    println!("HomeBot started in {} mode", config.general.mode);
    println!("Control loop running at {} Hz", config.general.update_rate_hz);
    
    // Main control loop
    let loop_duration = Duration::from_millis(1000 / config.general.update_rate_hz as u64);
    let mut last_time = Instant::now();
    
    while input_source.is_connected() {
        let loop_start = Instant::now();
        
        // Read sensor data
        let sensor_data = input_source.read_sensors()?;
        
        // Update motor control with new encoder values
        motor_control.update_encoders(sensor_data.left_encoder, sensor_data.right_encoder);
        
        // TODO: Implement your robot logic here
        // For now, just drive forward at 50% speed
        let motor_commands = motor_control.compute_motor_commands(0.5, 0.0);
        
        // Apply motor commands
        action_target.set_motor_speeds(
            motor_commands.left_speed,
            motor_commands.right_speed,
        )?;
        
        // Print status occasionally
        if loop_start.duration_since(last_time) > Duration::from_secs(1) {
            println!(
                "Encoders: L={}, R={} | Motors: L={}, R={}",
                sensor_data.left_encoder,
                sensor_data.right_encoder,
                motor_commands.left_speed,
                motor_commands.right_speed
            );
            last_time = loop_start;
        }
        
        // Maintain constant loop rate
        let elapsed = loop_start.elapsed();
        if elapsed < loop_duration {
            thread::sleep(loop_duration - elapsed);
        }
    }
    
    println!("Input source disconnected, shutting down...");
    action_target.stop_motors()?;
    
    Ok(())
}
