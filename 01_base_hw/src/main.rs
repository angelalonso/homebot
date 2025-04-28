mod motor;
use motor::Motor;
use std::error::Error;

#[tokio::main]  // Optional: Use async for more complex control
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize two motors (left and right)
    let mut left_motor = Motor::new(rppal::pwm::Channel::Pwm0, 17, 27)?;
    let mut right_motor = Motor::new(rppal::pwm::Channel::Pwm1, 22, 23)?;

    // Example: Move forward at 50% speed for 2 seconds
    left_motor.set_speed(0.5)?;
    right_motor.set_speed(0.5)?;
    std::thread::sleep(std::time::Duration::from_secs(2));

    // Stop motors
    left_motor.stop()?;
    right_motor.stop()?;

    Ok(())
}
