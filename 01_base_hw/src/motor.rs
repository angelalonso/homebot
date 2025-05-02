use gpiod::{Chip, Options};
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub struct L298N {
    in1: gpiod::Lines<gpiod::Output>,
    in2: gpiod::Lines<gpiod::Output>,
    en: gpiod::Lines<gpiod::Output>,
    pwm_chip: String,
    pwm_channel: u32,
}

impl L298N {
    pub fn new(
        gpio_chip: &str,
        in1_pin: u32,
        in2_pin: u32,
        en_pin: u32,
        pwm_chip: &str,
        pwm_channel: u32,
    ) -> Result<Self, Box<dyn Error>> {
        // Open GPIO chip
        let chip = Chip::new(gpio_chip)?;

        // Request GPIO lines as outputs
        let in1 = chip.request_lines(Options::output([in1_pin]).consumer("l298n-in1"))?;
        let in2 = chip.request_lines(Options::output([in2_pin]).consumer("l298n-in2"))?;
        let en = chip.request_lines(Options::output([en_pin]).consumer("l298n-en"))?;

        // Initialize PWM
        Self::init_pwm(pwm_chip, pwm_channel)?;

        Ok(L298N {
            in1,
            in2,
            en,
            pwm_chip: pwm_chip.to_string(),
            pwm_channel,
        })
    }

    fn init_pwm(chip: &str, channel: u32) -> Result<(), Box<dyn Error>> {
        let export_path = format!("/sys/class/pwm/{}/export", chip);
        if !Path::new(&export_path).exists() {
            File::create(export_path)?.write_all(format!("{}", channel).as_bytes())?;
            std::thread::sleep(std::time::Duration::from_millis(100));
        }

        // Set period to 1ms (1000000ns)
        File::create(format!("/sys/class/pwm/{}/pwm{}/period", chip, channel))?
            .write_all(b"1000000")?;

        // Enable PWM
        File::create(format!("/sys/class/pwm/{}/pwm{}/enable", chip, channel))?.write_all(b"1")?;

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

        // Set PWM duty cycle
        let duty_cycle = (speed.abs() * 1000000.0) as u32;
        let duty_path = format!(
            "/sys/class/pwm/{}/pwm{}/duty_cycle",
            self.pwm_chip, self.pwm_channel
        );
        File::create(duty_path)?.write_all(format!("{}", duty_cycle).as_bytes())?;

        Ok(())
    }

    pub fn stop(&mut self) -> Result<(), Box<dyn Error>> {
        // Stop PWM
        File::create(format!(
            "/sys/class/pwm/{}/pwm{}/duty_cycle",
            self.pwm_chip, self.pwm_channel
        ))?
        .write_all(b"0")?;

        // Set all control pins low
        self.in1.set_values([false])?;
        self.in2.set_values([false])?;
        self.en.set_values([false])?;

        Ok(())
    }
}
