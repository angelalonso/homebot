use std::ffi::CString;

use crate::bindings;
use crate::error::AppError;

// - Robot general functions
pub async fn find_port(time_step: i32) -> Result<PortConfig, AppError> {
    let distance_sensor_names = vec!["distance_sensor_eyes"];
    let distance_sensors: Vec<bindings::WbDeviceTag> = distance_sensor_names
        .iter()
        .map(|name| {
            let sensor: bindings::WbDeviceTag = robot_get_device(name);
            distance_sensor_enable(sensor, time_step);
            sensor
        })
        .collect();
    let result = PortConfig::Multiple(distance_sensors);
    Ok(result)
}

pub async fn find_distance_sensor(time_step: i32, name: &str) -> Result<String, AppError> {
    let sensor: bindings::WbDeviceTag = robot_get_device(name);
    distance_sensor_enable(sensor, time_step);
    Ok(sensor.to_string()) // TODO: make back and forth u16 to string
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
pub fn get_sensors_ids(distance_sensor_names: Vec<&str>, time_step: i32) -> Vec<u16> {
    let distance_sensors = distance_sensor_names
        .iter()
        .map(|name| {
            let sensor: bindings::WbDeviceTag = robot_get_device(name);
            distance_sensor_enable(sensor, time_step);
            sensor
        })
        .collect();
    return distance_sensors;
}

pub fn get_distance_sensor_id(distance_sensor_name: &str, time_step: i32) -> u16 {
    let distance_sensor: bindings::WbDeviceTag = robot_get_device(distance_sensor_name);
    distance_sensor_enable(distance_sensor, time_step);
    return distance_sensor;
}

pub fn read_distance(in_port: PortConfig, time_step: i32) -> Vec<f64> {
    //TODO: get sensors ids outside and only once
    //let distance_values: Vec<_> = get_sensors_ids(in_port, time_step)
    //    .iter()
    let distance_values: Vec<_> = in_port
        .as_string_iter()
        .map(|sensor| distance_sensor_get_value(sensor.parse::<u16>().unwrap()))
        .collect();
    return distance_values;
}

/*
pub fn read_distance(sensor: &str, time_step: i32) -> f64 {
    println!("----- testing {:?}", sensor);
    let sensor_id = match sensor.parse::<u16>() {
        Ok(num) => {
            println!("OK: {}", num);
            num
        },
        Err(e) => {
            eprintln!("Error: {} {}", sensor, e);
            0
        }
    };
    let distance_values = distance_sensor_get_value(sensor_id);
    return distance_values;
}
*/

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
