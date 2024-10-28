pub mod loggin;
pub mod sim_action;
pub mod sim_brain;
pub mod sim_input;
pub mod sim_output;
pub mod sim_reactionset;

#[cfg(any(feature = "sim", feature = "test"))]
pub mod sim;

#[cfg(any(feature = "sim", feature = "test"))]
pub mod bindings {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(improper_ctypes)]
    include!("sim_bindings.rs");
}

#[cfg(any(feature = "sim", feature = "test"))]
use bindings::WbDeviceTag;
#[cfg(any(feature = "sim", feature = "test"))]
use std::ffi::CString;

// ////the clisim alternatives are on clibot.rs

#[cfg(any(feature = "sim", feature = "test"))]
pub fn wb_distance_sensor_enable(tag: WbDeviceTag, sampling_period: i32) {
    unsafe {
        crate::bindings::wb_distance_sensor_enable(tag, sampling_period);
    }
}

#[cfg(any(feature = "sim", feature = "test"))]
pub fn wb_distance_sensor_get_value(tag: WbDeviceTag) -> f64 {
    unsafe { crate::bindings::wb_distance_sensor_get_value(tag) }
}

#[cfg(any(feature = "sim", feature = "test"))]
pub fn wb_motor_set_position(device: WbDeviceTag, position: f64) {
    unsafe { crate::bindings::wb_motor_set_position(device, position) }
}

#[cfg(any(feature = "sim", feature = "test"))]
pub fn wb_motor_set_velocity(device: WbDeviceTag, velocity: f64) {
    unsafe { crate::bindings::wb_motor_set_velocity(device, velocity) }
}

#[cfg(any(feature = "sim", feature = "test"))]
pub fn wb_robot_get_device(id: &str) -> WbDeviceTag {
    let name = CString::new(id).expect("CString::new failed");
    unsafe { crate::bindings::wb_robot_get_device(name.as_ptr()) }
}

#[cfg(any(feature = "sim", feature = "test"))]
pub fn wb_robot_cleanup() {
    unsafe { crate::bindings::wb_robot_cleanup() }
}

#[cfg(any(feature = "sim", feature = "test"))]
pub fn wb_robot_init() {
    unsafe {
        crate::bindings::wb_robot_init();
    }
}

#[cfg(any(feature = "sim", feature = "test"))]
pub fn wb_robot_step(step: i32) -> i32 {
    unsafe { crate::bindings::wb_robot_step(step) }
}
