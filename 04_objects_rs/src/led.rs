use gpio_cdev::{Chip, LineRequestFlags};
use crate::error::AppError;

pub struct Led {
    line: gpio_cdev::LineHandle,
}

impl Led {
    pub fn new(chip: &mut Chip, pin: u32) -> Result<Self, AppError> {
        let line = chip.get_line(pin)?
            .request(LineRequestFlags::OUTPUT, 0, "distance_led")?;
        Ok(Self { line })
    }

    pub fn set(&mut self, on: bool) -> Result<(), AppError> {
        self.line.set_value(if on { 1 } else { 0 })?;
        Ok(())
    }
}
