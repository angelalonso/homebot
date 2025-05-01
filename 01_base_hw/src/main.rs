mod motor;
use motor::Motor;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Initializing motors...");

    // Initialize two motors (left and right)
    let mut left_motor = Motor::new("gpiochip0", 17, 27, "pwmchip0", 0)?;
    let mut right_motor = Motor::new("gpiochip0", 22, 23, "pwmchip0", 1)?;

    println!("Moving forward at 50% speed for 2 seconds...");
    left_motor.set_speed(0.5)?;
    right_motor.set_speed(0.5)?;
    std::thread::sleep(std::time::Duration::from_secs(2));

    println!("Stopping motors...");
    left_motor.stop()?;
    right_motor.stop()?;

    Ok(())
}
