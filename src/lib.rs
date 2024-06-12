pub mod movement;
pub mod queue;

#[cfg(feature = "simulate")]
pub mod bindings {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(improper_ctypes)]
    include!("bindings.rs");
}

#[cfg(feature = "simulate")]
use bindings::WbDeviceTag;
#[cfg(feature = "simulate")]
use std::ffi::CString;

#[cfg(feature = "simulate")]
pub fn wb_distance_sensor_enable(tag: WbDeviceTag, sampling_period: i32) {
    unsafe {
        crate::bindings::wb_distance_sensor_enable(tag, sampling_period);
    }
}

#[cfg(feature = "simulate")]
pub fn wb_distance_sensor_get_value(tag: WbDeviceTag) -> f64 {
    unsafe { crate::bindings::wb_distance_sensor_get_value(tag) }
}

#[cfg(feature = "simulate")]
pub fn wb_motor_set_position(device: WbDeviceTag, position: f64) {
    unsafe { crate::bindings::wb_motor_set_position(device, position) }
}

#[cfg(feature = "simulate")]
pub fn wb_motor_set_velocity(device: WbDeviceTag, velocity: f64) {
    unsafe { crate::bindings::wb_motor_set_velocity(device, velocity) }
}

#[cfg(feature = "simulate")]
pub fn wb_robot_get_device(id: &str) -> WbDeviceTag {
    let name = CString::new(id).expect("CString::new failed");
    unsafe { crate::bindings::wb_robot_get_device(name.as_ptr()) }
}

#[cfg(feature = "simulate")]
pub fn wb_robot_cleanup() {
    unsafe { crate::bindings::wb_robot_cleanup() }
}

#[cfg(feature = "simulate")]
pub fn wb_robot_init() {
    unsafe {
        crate::bindings::wb_robot_init();
    }
}

#[cfg(feature = "simulate")]
pub fn wb_robot_step(step: i32) -> i32 {
    unsafe { crate::bindings::wb_robot_step(step) }
}

#[cfg(not(feature = "simulate"))]
pub mod robot;
