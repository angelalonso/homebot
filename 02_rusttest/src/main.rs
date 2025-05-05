mod motor;
use motor::L298N;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize L298N controller
    // Parameters: gpio_chip, in1_pin, in2_pin, en_pin, pwm_chip, pwm_channel
    println!("A");
    let mut motor = L298N::new("gpiochip0", 17, 27, 22, "pwmchip0", 0)?;

    // Example usage:
    println!("Moving forward at 50% speed for 2 seconds");
    motor.set_speed(0.5)?;
    std::thread::sleep(std::time::Duration::from_secs(2));

    println!("Moving backward at 30% speed for 1 second");
    motor.set_speed(-0.3)?;
    std::thread::sleep(std::time::Duration::from_secs(1));

    println!("Stopping motor");
    motor.stop()?;

    Ok(())
}
