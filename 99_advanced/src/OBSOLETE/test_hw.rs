pub fn ts_robot_init() {
    ();
}

pub fn ts_distance_sensor_enable(_tag: u16, _sampling_period: i32) {
    ();
}

pub fn ts_distance_sensor_get_value(_tag: u16) -> f64 {
    0.0
}

pub fn ts_motor_set_position(_device: String, _position: f64) {
    ();
}

pub fn ts_motor_set_velocity(_device: String, _velocity: f64) {
    ();
}

pub fn ts_robot_get_device(_id: &str) -> u16 {
    0
}

pub fn ts_robot_cleanup() {
    ();
}

pub fn ts_robot_step(_step: i32) -> i32 {
    0
}
