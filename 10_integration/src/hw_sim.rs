use std::ffi::CString;

use crate::bindings;
use crate::error::AppError;

// - Robot general functions
pub async fn get_serial_port(time_step: i32) -> Result<(String, Vec<u16>), AppError> {
    let distance_sensor_names = vec!["distance_sensor_eyes"];
    let mut distance_sensors: Vec<bindings::WbDeviceTag> = [].to_vec();
    distance_sensors = distance_sensor_names
        .iter()
        .map(|name| {
            let sensor: bindings::WbDeviceTag = robot_get_device(name);
            distance_sensor_enable(sensor, time_step);
            println!("1 here");
            sensor
        })
        .collect();  

    Ok(("".to_string(), distance_sensors))
}

pub fn read_distance(_serial_port: &str, sensor_ids: Vec<u16>, time_step: i32) -> Vec<f64> {
    let distance_values = sensor_ids
        .iter()
        .map(|sensor| distance_sensor_get_value(*sensor))
        .collect();
    return distance_values;
}

pub fn robot_get_device(id: &str) -> bindings::WbDeviceTag {
    let name = CString::new(id).expect("CString::new failed");
    unsafe { bindings::wb_robot_get_device(name.as_ptr()) }
}

pub fn robot_init() {
    unsafe {
        bindings::wb_robot_init();
    }
}

pub fn robot_step(step: i32) -> i32 {
    unsafe { bindings::wb_robot_step(step) }
}

pub fn robot_cleanup() {
    unsafe { bindings::wb_robot_cleanup() }
}

// - Sensors functions
pub fn distance_sensor_get_value(tag: bindings::WbDeviceTag) -> f64 {
    unsafe { bindings::wb_distance_sensor_get_value(tag) }
}

pub fn distance_sensor_enable(tag: bindings::WbDeviceTag, sampling_period: i32) {
    unsafe {
        bindings::wb_distance_sensor_enable(tag, sampling_period);
    }
}

// - LED functions

pub fn led_disable(tag: bindings::WbDeviceTag) {
    unsafe {
        bindings::wb_led_set(tag, 0);
    }
}

pub fn led_enable(tag: bindings::WbDeviceTag) {
    unsafe {
        bindings::wb_led_set(tag, 1);
    }
}

// - Motor functions
pub fn motor_set_position(device: bindings::WbDeviceTag, position: f64) {
    unsafe { bindings::wb_motor_set_position(device, position) }
}

pub fn motor_set_velocity(device: bindings::WbDeviceTag, velocity: f64) {
    unsafe { bindings::wb_motor_set_velocity(device, velocity) }
}
