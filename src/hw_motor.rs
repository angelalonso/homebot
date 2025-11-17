/* TO BE DONE: 
 *
 * This does not work because it tries to be loaded on sim mode and there's nothing behind it
 * */
use crate::bindings;
use crate::error::AppError;

pub enum MotorMode {
    Real {
        chip: gpio_cdev::Chip,
        pin1: u32,
        pin2: u32,
        pin_enable: u32,
    },
    Test,
    Sim {
        wb_device: bindings::WbDeviceTag,
    },
}

pub struct Motor {
    mode: MotorMode,
}

impl Motor {
    pub fn new_real(chip: gpio_cdev::Chip, pin1: u32, pin2: u32, pin_enable: u32) -> Result<Self, AppError> {
        Ok(Self {
            mode: MotorMode::Real { chip, pin1, pin2, pin_enable },
        })
    }
    
    pub fn new_test() -> Result<Self, AppError> {
        Ok(Self {
            mode: MotorMode::Test,
        })
    }
    
    pub fn new_sim(wb_device: bindings::WbDeviceTag) -> Result<Self, AppError> {
        Ok(Self {
            mode: MotorMode::Sim { wb_device },
        })
    }
    
    pub fn set_velocity(&mut self, velocity: f64) -> Result<(), Box<dyn std::error::Error>> {
        match &mut self.mode {
            MotorMode::Real { chip, pin1, pin2, pin_enable } => {
                let mut motor = bindings::Motor::new(chip, *pin1, *pin2, *pin_enable)?;
                let _ = motor.set_speed(velocity as i8);
                Ok(())
            }
            MotorMode::Test => {
                println!("Test mode: Setting velocity to {}", velocity);
                Ok(())
            }
            MotorMode::Sim { wb_device } => {
                println!("Sim mode: Setting velocity to {} using Webots device", velocity);
                unsafe { bindings::wb_motor_set_velocity(*wb_device, velocity) }
                Ok(())
            }
        }
    }
}
