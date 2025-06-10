pub mod bindings {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(improper_ctypes)]
    include!("sim_bindings.rs");
}

use crate::error::AppError;

use crate::sim_bindings::WbDeviceTag;

pub async fn find_port(time_step: i32) -> Result<String, AppError> {
    let distance_sensor_names = vec!["distance_sensor_eyes"];
    let mut distance_sensors: Vec<WbDeviceTag> = [].to_vec();
    distance_sensors = distance_sensor_names
        .iter()
        .map(|name| {
            let sensor: WbDeviceTag = crate::sim_hw::wb_robot_get_device(name);
            crate::sim_hw::wb_distance_sensor_enable(sensor, time_step);
            sensor
        })
        .collect();
    let result = String::from_utf16(&distance_sensors).unwrap_or_else(|err| {
        // Handle invalid UTF-16 sequences
        String::from_utf16_lossy(&distance_sensors)
    });
    Ok(result)
}

// - Robot functions that webots need

pub fn robot_init() {
    unsafe {
        crate::sim_bindings::wb_robot_init();
    }
}

pub fn robot_cleanup() {
    unsafe { bindings::wb_robot_cleanup() }
}

pub fn robot_step(step: i32) -> i32 {
    unsafe { bindings::wb_robot_step(step) }
}
