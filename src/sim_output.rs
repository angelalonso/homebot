#[derive(Debug, Clone)]
pub struct Output {
    sensor: String,
    sensor_prio: u8,
    motor_l: f32,
    motor_l_prio: u8,
    motor_r: f32,
    motor_r_prio: u8,
}

impl Output {
    pub fn new() -> Self {
        Self {
            sensor: "on".to_string(),
            sensor_prio: 0,
            motor_l: 0.0,
            motor_l_prio: 0,
            motor_r: 0.0,
            motor_r_prio: 0,
        }
    }

    pub fn set_sensor(&mut self, value: String, prio: u8) {
        self.sensor = value;
        self.sensor_prio = prio;
    }

    pub fn set_motor_l(&mut self, value: f32, prio: u8) {
        self.motor_l_prio = prio;
    }

    pub fn set_motor_r(&mut self, value: f32, prio: u8) {
        self.motor_r_prio = prio;
    }

    pub fn get_sensor(&mut self) -> (String, u8) {
        return (self.sensor.clone(), self.sensor_prio);
    }

    pub fn get_motor_l(&mut self) -> (f32, u8) {
        return (self.motor_l, self.motor_l_prio);
    }

    pub fn get_motor_r(&mut self) -> (f32, u8) {
        return (self.motor_r, self.motor_r_prio);
    }
}
