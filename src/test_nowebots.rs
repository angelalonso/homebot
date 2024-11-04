pub fn wb_distance_sensor_enable(_tag: WbDeviceTag, _sampling_period: i32) {
    ();
}

pub fn wb_distance_sensor_get_value(_tag: WbDeviceTag) -> f64 {
    0.0
}

pub fn wb_motor_set_position(_device: WbDeviceTag, _position: f64) {
    ();
}

pub fn wb_motor_set_velocity(_device: WbDeviceTag, _velocity: f64) {
    ();
}

pub fn wb_robot_get_device(_id: &str) -> WbDeviceTag {
    ();
}

pub fn wb_robot_cleanup() {
    ();
}

pub fn wb_robot_init() {
    ();
}

pub fn wb_robot_step(_step: i32) -> i32 {
    0
}

