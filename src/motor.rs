// motor.rs
use std::collections::BTreeMap;
use crate::loggin::Log;

// Common motor trait that all implementations must fulfill
pub trait MotorController {
    fn set_speed(&mut self, speed: f32);
    fn get_speed(&self) -> f32;
    fn get_priority(&self) -> u8;
    fn set_priority(&mut self, prio: u8);
}

// Common output structure that works across all modes
pub struct Output<ML, MR>
where
    ML: MotorController,
    MR: MotorController,
{
    pub sensor: String,
    pub sensor_prio: u8,
    pub motor_l: ML,
    pub motor_r: MR,
}

impl<ML, MR> Output<ML, MR>
where
    ML: MotorController,
    MR: MotorController,
{
    pub fn set_sensor(&mut self, value: String, prio: u8) {
        self.sensor = value;
        self.sensor_prio = prio;
    }

    pub fn set_motor_l(&mut self, value: f32, prio: u8) {
        self.motor_l.set_speed(value);
        self.motor_l.set_priority(prio);
    }

    pub fn set_motor_r(&mut self, value: f32, prio: u8) {
        self.motor_r.set_speed(value);
        self.motor_r.set_priority(prio);
    }

    pub fn get_sensor(&self) -> (String, u8) {
        (self.sensor.clone(), self.sensor_prio)
    }

    pub fn get_motor_l_prio(&self) -> u8 {
        self.motor_l.get_priority()
    }

    pub fn get_motor_r_prio(&self) -> u8 {
        self.motor_r.get_priority()
    }
}
