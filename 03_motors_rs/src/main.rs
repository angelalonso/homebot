use gpio_cdev::{Chip, LineRequestFlags};
use std::error::Error;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

// GPIO Pins (BCM numbering)
const LED_PIN: u32 = 4;
const MOTOR_A_ENABLE: u32 = 22;
const MOTOR_A_IN1: u32 = 17;
const MOTOR_A_IN2: u32 = 27;
const MOTOR_B_ENABLE: u32 = 25;
const MOTOR_B_IN1: u32 = 23;
const MOTOR_B_IN2: u32 = 24;

const STOP_DISTANCE_CM: u32 = 20;

struct MotorController {
    enable: gpio_cdev::LineHandle,
    in1: gpio_cdev::LineHandle,
    in2: gpio_cdev::LineHandle,
}

impl MotorController {
    fn new(enable_pin: u32, in1_pin: u32, in2_pin: u32) -> Result<Self, Box<dyn Error>> {
        let mut chip = Chip::new("/dev/gpiochip0")?;

        // Request lines with output direction
        let enable = chip
            .get_line(enable_pin)?
            .request(LineRequestFlags::OUTPUT, 0, "MOTOR_EN")?;
        let in1 = chip
            .get_line(in1_pin)?
            .request(LineRequestFlags::OUTPUT, 0, "MOTOR_IN1")?;
        let in2 = chip
            .get_line(in2_pin)?
            .request(LineRequestFlags::OUTPUT, 0, "MOTOR_IN2")?;

        Ok(Self { enable, in1, in2 })
    }

    fn set_speed(&mut self, speed: i32) -> Result<(), Box<dyn Error>> {
        if speed > 0 {
            // Forward
            self.in1.set_value(1)?;
            self.in2.set_value(0)?;
            // Simulate PWM by toggling (crude implementation)
            self.enable.set_value(1)?;
        } else if speed < 0 {
            // Reverse
            self.in1.set_value(0)?;
            self.in2.set_value(1)?;
            // Simulate PWM by toggling
            self.enable.set_value(1)?;
        } else {
            // Stop
            self.in1.set_value(0)?;
            self.in2.set_value(0)?;
            self.enable.set_value(0)?;
        }
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // Setup Ctrl-C handler
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    // Initialize LED
    let mut led_chip = Chip::new("/dev/gpiochip0")?;
    let led = led_chip
        .get_line(LED_PIN)?
        .request(LineRequestFlags::OUTPUT, 0, "LED")?;

    // Initialize Motors
    let mut motor_a = MotorController::new(MOTOR_A_ENABLE, MOTOR_A_IN1, MOTOR_A_IN2)?;
    let mut motor_b = MotorController::new(MOTOR_B_ENABLE, MOTOR_B_IN1, MOTOR_B_IN2)?;

    // Start motors forward at 50% (simulated)
    motor_a.set_speed(50)?;
    motor_b.set_speed(50)?;

    while running.load(Ordering::SeqCst) {
        // Your distance sensor logic would go here
        // For now, just toggle LED for testing
        led.set_value(1)?;
        thread::sleep(Duration::from_millis(500));
        led.set_value(0)?;
        thread::sleep(Duration::from_millis(500));
    }

    println!("Stopping...");

    // Cleanup
    motor_a.set_speed(0)?;
    motor_b.set_speed(0)?;
    led.set_value(0)?;

    Ok(())
}
