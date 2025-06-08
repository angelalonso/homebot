use crate::error::AppError;
use gpio_cdev::{Chip, LineRequestFlags};

pub struct Motor {
    enable: gpio_cdev::LineHandle,
    in1: gpio_cdev::LineHandle,
    in2: gpio_cdev::LineHandle,
}

impl Motor {
    pub fn new(
        chip: &mut Chip,
        enable_pin: u32,
        in1_pin: u32,
        in2_pin: u32,
    ) -> Result<Self, AppError> {
        Ok(Self {
            enable: chip.get_line(enable_pin)?.request(
                LineRequestFlags::OUTPUT,
                0,
                "motor_enable",
            )?,
            in1: chip
                .get_line(in1_pin)?
                .request(LineRequestFlags::OUTPUT, 0, "motor_in1")?,
            in2: chip
                .get_line(in2_pin)?
                .request(LineRequestFlags::OUTPUT, 0, "motor_in2")?,
        })
    }

    pub fn set_speed(&mut self, speed: i8) -> Result<(), AppError> {
        match speed {
            s if s > 0 => {
                // Forward
                self.in1.set_value(1)?;
                self.in2.set_value(0)?;
                self.enable.set_value(1)?;
            }
            s if s < 0 => {
                // Reverse
                self.in1.set_value(0)?;
                self.in2.set_value(1)?;
                self.enable.set_value(1)?;
            }
            _ => {
                // Stop
                self.enable.set_value(0)?;
            }
        }
        Ok(())
    }
}
