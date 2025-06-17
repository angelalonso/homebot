pub mod bindings {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(improper_ctypes)]
    include!("bindings_sim.rs");
}
use crate::error::AppError;
use bindings::WbDeviceTag;

use std::ffi::CString;

pub async fn find_port(time_step: i32) -> Result<String, AppError> {
    let distance_sensor_names = vec!["distance_sensor_eyes"];
    let distance_sensors: Vec<WbDeviceTag> = distance_sensor_names
        .iter()
        .map(|name| {
            let sensor: WbDeviceTag = crate::hw_sim::wb_robot_get_device(name);
            crate::hw_sim::wb_distance_sensor_enable(sensor, time_step);
            sensor
        })
        .collect();
    let result = String::from_utf16(&distance_sensors).unwrap_or_else(|_err| {
        // Handle invalid UTF-16 sequences
        String::from_utf16_lossy(&distance_sensors)
    });
    Ok(result)
}

// - Robot functions that webots need

pub fn distance_sensor_get_value(tag: WbDeviceTag) -> f64 {
    unsafe { bindings::wb_distance_sensor_get_value(tag) }
}
// - My Own START

pub fn get_sensors_ids(distance_sensor_names: Vec<&str>, time_step: i32) -> Vec<u16> {
    let distance_sensors = distance_sensor_names
        .iter()
        .map(|name| {
            let sensor: WbDeviceTag = wb_robot_get_device(name);
            wb_distance_sensor_enable(sensor, time_step);
            sensor
        })
        .collect();
    return distance_sensors;
}

// - My Own END
// - Previous functions START

pub fn wb_robot_get_device(id: &str) -> WbDeviceTag {
    let name = CString::new(id).expect("CString::new failed");
    unsafe { bindings::wb_robot_get_device(name.as_ptr()) }
}

pub fn wb_distance_sensor_enable(tag: WbDeviceTag, sampling_period: i32) {
    unsafe {
        bindings::wb_distance_sensor_enable(tag, sampling_period);
    }
}
// - Previous functions END

pub fn wbr_led_disable(tag: WbDeviceTag) {
    unsafe {
        bindings::wb_led_set(tag, 0);
    }
}

pub fn wbr_led_enable(tag: WbDeviceTag) {
    unsafe {
        bindings::wb_led_set(tag, 1);
    }
}

pub fn wb_motor_set_position(device: WbDeviceTag, position: f64) {
    unsafe { bindings::wb_motor_set_position(device, position) }
}

pub fn wb_motor_set_velocity(device: WbDeviceTag, velocity: f64) {
    unsafe { bindings::wb_motor_set_velocity(device, velocity) }
}

// - Robot mockups required TODO: get something useful from here
pub fn robot_init() {
    unsafe {
        bindings::wb_robot_init();
    }
}

pub fn robot_cleanup() {
    unsafe { bindings::wb_robot_cleanup() }
}

pub fn robot_step(step: i32) -> i32 {
    unsafe { bindings::wb_robot_step(step) }
}

