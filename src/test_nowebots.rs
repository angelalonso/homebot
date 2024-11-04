pub fn wb_distance_sensor_enable(_tag: String, _sampling_period: i32) {
    ();
}

pub fn wb_distance_sensor_get_value(_tag: String) -> f64 {
    0.0
}

pub fn wb_motor_set_position(_device: String, _position: f64) {
    ();
}

pub fn wb_motor_set_velocity(_device: String, _velocity: f64) {
    ();
}

pub fn wb_robot_get_device(_id: &str) {
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
