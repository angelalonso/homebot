pub fn initialize() {
    println!("Mocking initialize of pigpio");
}

#[allow(non_camel_case_types)]
pub struct pwm {
    _on: bool,
}

impl pwm {
    pub fn pwm(_pin_enabler: u32, _value: u32) -> Result<(), &'static str> {
        Ok(())
    }
    pub fn set_pwm_frequency(_pin_enabler: u32, _freq: u32) -> Result<(), &'static str> {
        Ok(())
    }
    pub fn set_pwm_range(_pin_enabler: u32, _range: u32) -> Result<(), &'static str> {
        Ok(())
    }
}
