mod arduino;
mod error;
mod led;
mod motor;

use arduino::{find_arduino, Arduino};
use error::AppError;
use led::Led;
use motor::Motor;
use tokio::time::{sleep, Duration};

const LED_PIN: u32 = 4;
const MOTOR_A_PINS: (u32, u32, u32) = (17, 27, 22); // (in1, in2, enable)
const MOTOR_B_PINS: (u32, u32, u32) = (23, 24, 25);
const STOP_DISTANCE_CM: f32 = 20.0;

struct Robot {
    arduino: Arduino,
    led: Led,
    motor_a: Motor,
    motor_b: Motor,
}

impl Robot {
    async fn new() -> Result<Self, AppError> {
        let mut chip = gpio_cdev::Chip::new("/dev/gpiochip0")?;
        let port_path = find_arduino().await?;

        Ok(Self {
            arduino: Arduino::new(&port_path).await?,
            led: Led::new(&mut chip, LED_PIN)?,
            motor_a: Motor::new(&mut chip, MOTOR_A_PINS.0, MOTOR_A_PINS.1, MOTOR_A_PINS.2)?,
            motor_b: Motor::new(&mut chip, MOTOR_B_PINS.0, MOTOR_B_PINS.1, MOTOR_B_PINS.2)?,
        })
    }

    async fn run(&mut self) -> Result<(), AppError> {
        // Start motors
        self.motor_a.set_speed(50)?;
        self.motor_b.set_speed(50)?;

        loop {
            if let Some(distance) = self.arduino.read_distance().await? {
                let should_stop = distance < STOP_DISTANCE_CM;
                self.led.set(should_stop)?;

                if should_stop {
                    self.motor_a.set_speed(0)?;
                    self.motor_b.set_speed(0)?;
                } else {
                    self.motor_a.set_speed(50)?;
                    self.motor_b.set_speed(50)?;
                }

                println!(
                    "Distance: {:.1}cm - Motors: {}",
                    distance,
                    if should_stop { "STOPPED" } else { "RUNNING" }
                );
            }

            sleep(Duration::from_millis(10)).await;
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), AppError> {
    let mut robot = Robot::new().await?;
    robot.run().await
}
