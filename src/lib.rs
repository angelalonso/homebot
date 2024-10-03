pub mod brain;
pub mod loggin;
pub mod queue;

#[cfg(not(feature = "simulate"))]
pub mod clibot;
// //#[cfg(not(feature = "simulate"))]
// //pub mod clisimulate;
#[cfg(feature = "simulate")]
pub mod simulate;

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

// ////the clisimulate alternatives are on clibot.rs

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

#[cfg(test)]
mod tests {
    use crate::queue::add;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
