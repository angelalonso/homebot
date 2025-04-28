pub mod bindings {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(improper_ctypes)]
    include!("live_bindings.rs");
}

pub fn hw_robot_init() {
    ();
}

pub fn hw_led_enable(_tag: u16, pin: u8) {
    let led = bindings::GPIOLed::new(pin);
    led.on();
}

pub fn hw_led_disable(_tag: u16, pin: u8) {
    let led = bindings::GPIOLed::new(pin);
    led.off();
}

pub fn hw_distance_sensor_enable(_tag: u16, _sampling_period: i32) {
    ();
}

pub fn hw_distance_sensor_get_value(_tag: u16) -> f64 {
    0.0
}

pub fn hw_motor_set_position(_device: String, _position: f64) {
    ();
}

pub fn hw_motor_set_velocity(_device: String, _velocity: f64) {
    ();
}

pub fn hw_robot_get_device(_id: &str) -> u16 {
    0
}

pub fn hw_robot_cleanup() {
    ();
}

pub fn hw_robot_step(_step: i32) -> i32 {
    0
}
