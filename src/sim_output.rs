use crate::loggin::Log;

#[derive(Debug, Clone)]
pub struct Output {
    sensor: String,
    sensor_prio: u8,
    l_motor: u16,
    motor_l: f32,
    motor_l_prio: u8,
    r_motor: u16,
    motor_r: f32,
    motor_r_prio: u8,
}

impl Output {
    pub fn new(log: Log) -> Self {
        log.info("Loading motors...");
        let infinity = f64::INFINITY;
        let left_motor = crate::wb_robot_get_device("left_wheel_motor");
        let right_motor = crate::wb_robot_get_device("right_wheel_motor");
        crate::wb_motor_set_position(left_motor, infinity);
        crate::wb_motor_set_position(right_motor, infinity);
        crate::wb_motor_set_velocity(left_motor, 0.0);
        crate::wb_motor_set_velocity(right_motor, 0.0);
        Self {
            sensor: "on".to_string(),
            sensor_prio: 0,
            l_motor: left_motor,
            motor_l: 0.0,
            motor_l_prio: 0,
            r_motor: right_motor,
            motor_r: 0.0,
            motor_r_prio: 0,
        }
    }

    pub fn set_sensor(&mut self, value: String, prio: u8) {
        self.sensor = value;
        self.sensor_prio = prio;
    }

    pub fn set_motor_l(&mut self, value: f32, prio: u8) {
        //let max_speed = 6.28;
        let max_speed = 1.00;
        crate::wb_motor_set_velocity(self.l_motor, (value * max_speed).into());
        self.motor_l = value;
        self.motor_l_prio = prio;
    }

    pub fn set_motor_r(&mut self, value: f32, prio: u8) {
        let max_speed = 1.00;
        crate::wb_motor_set_velocity(self.r_motor, (value * max_speed).into());
        self.motor_r = value;
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
