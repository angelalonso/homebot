use crate::hw::bindings::Motor;
use gpio_cdev::Chip;

pub mod bindings {
    include!("live_bindings.rs");
}

// use bindings::WbDeviceTag;
// use std::ffi::CString;

// - My Own START

pub fn get_sensors_ids(distance_sensor_names: Vec<&str>, time_step: i32) -> Vec<u16> {
    let distance_sensors = vec![];
    return distance_sensors;
}

// - My Own END

//pub fn hw_led_enable(_tag: u16, pin: u8) {
//    let led = bindings::GPIOLed::new(pin);
//    led.on();
//}
//
//pub fn hw_led_disable(_tag: u16, pin: u8) {
//    let led = bindings::GPIOLed::new(pin);
//    led.off();
//}

pub fn hw_distance_sensor_enable(_tag: u16, _sampling_period: i32) {
    ();
}

pub fn hw_motor_set_position(_device: String, _position: f64) {
    ();
}

pub fn hw_motor_set_velocity(
    pins: (u32, u32, u32),
    velocity: f64,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut chip: Chip = gpio_cdev::Chip::new("/dev/gpiochip0")?;
    // live_bindings, also check 04 for how we do that
    let mut motor = Motor::new(&mut chip, pins.0, pins.1, pins.2)?;
    motor.set_speed(velocity as i8);
    Ok(())
}

pub fn hw_robot_get_device(_id: &str) -> u16 {
    0
}

pub fn hw_robot_cleanup() {
    ();
}

// - Latest mockups required TODO: get something useful from here

pub fn robot_init() {
    ();
}

pub fn robot_cleanup() {
    ();
}

pub fn robot_step(_step: i32) -> i32 {
    0
}

pub fn distance_sensor_get_value(_tag: u16) -> f64 {
    0.0
}
