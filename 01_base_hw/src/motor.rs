use gpiod::{Chip, Direction, Options};
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub struct Motor {
    in1: gpiod::Lines<gpiod::Output>,
    in2: gpiod::Lines<gpiod::Output>,
    pwm_chip: String,
    pwm_channel: u32,
}

impl Motor {
    pub fn new(
        gpio_chip: &str,
        in1_pin: u32,
        in2_pin: u32,
        pwm_chip: &str,
        pwm_channel: u32,
    ) -> Result<Self, Box<dyn Error>> {
        // Open GPIO chip
        let chip = Chip::new(gpio_chip)?;

        // Request GPIO lines as outputs
        let in1_options = Options::output([in1_pin]).consumer("motor-in1");
        let in1 = chip.request_lines(in1_options)?;

        let in2_options = Options::output([in2_pin]).consumer("motor-in2");
        let in2 = chip.request_lines(in2_options)?;

        // Initialize PWM (Linux sysfs interface)
        Self::init_pwm(pwm_chip, pwm_channel)?;

        Ok(Motor {
            in1,
            in2,
            pwm_chip: pwm_chip.to_string(),
            pwm_channel,
        })
    }

    fn init_pwm(chip: &str, channel: u32) -> Result<(), Box<dyn Error>> {
        // Export PWM channel
        let export_path = format!("/sys/class/pwm/{}/export", chip);
        if !Path::new(&export_path).exists() {
            let mut export_file = File::create(export_path)?;
            write!(export_file, "{}", channel)?;

            // Wait for PWM to be ready
            std::thread::sleep(std::time::Duration::from_millis(100));
        }

        // Set PWM period (1ms = 1000000ns)
        let period_path = format!("/sys/class/pwm/{}/pwm{}/period", chip, channel);
        let mut period_file = File::create(period_path)?;
        write!(period_file, "1000000")?;

        // Enable PWM
        let enable_path = format!("/sys/class/pwm/{}/pwm{}/enable", chip, channel);
        let mut enable_file = File::create(enable_path)?;
        write!(enable_file, "1")?;

        Ok(())
    }

    pub fn set_speed(&mut self, speed: f64) -> Result<(), Box<dyn Error>> {
        let speed = speed.clamp(-1.0, 1.0);

        // Set direction
        if speed >= 0.0 {
            self.in1.set_values([true])?;
            self.in2.set_values([false])?;
        } else {
            self.in1.set_values([false])?;
            self.in2.set_values([true])?;
        }

        // Set PWM duty cycle (absolute value)
        let duty_cycle = (speed.abs() * 1000000.0) as u32;
        let duty_path = format!(
            "/sys/class/pwm/{}/pwm{}/duty_cycle",
            self.pwm_chip, self.pwm_channel
        );
        let mut duty_file = File::create(duty_path)?;
        write!(duty_file, "{}", duty_cycle)?;

        Ok(())
    }

    pub fn stop(&mut self) -> Result<(), Box<dyn Error>> {
        // Set duty cycle to 0
        let duty_path = format!(
            "/sys/class/pwm/{}/pwm{}/duty_cycle",
            self.pwm_chip, self.pwm_channel
        );
        let mut duty_file = File::create(duty_path)?;
        write!(duty_file, "0")?;

        // Set GPIOs to low
        self.in1.set_values([false])?;
        self.in2.set_values([false])?;

        Ok(())
    }
}
