use crate::loggin::Log;

#[derive(Debug, Clone, PartialEq)]
pub struct Output {
    sensor: String,
    sensor_prio: u8,
    #[allow(dead_code)]
    motor_l_pins: (u32, u32, u32), // only used in sim mode
    motor_l_vel: f32,
    motor_l_prio: u8,
    #[allow(dead_code)]
    motor_r: u16, // only used in sim mode
    motor_r_vel: f32,
    motor_r_prio: u8,
}

impl Output {
    pub fn new(log: Log) -> Self {
        let left_wheel_motor: (u32, u32, u32);
        let right_wheel_motor: u16;

        log.info("Loading motors...");
        let _infinity = f64::INFINITY;

        left_wheel_motor = (17, 27, 22);
        right_wheel_motor = 2;

        Self {
            sensor: "on".to_string(),
            sensor_prio: 0,
            motor_l_pins: left_wheel_motor,
            motor_l_vel: 0.0,
            motor_l_prio: 0,
            motor_r: right_wheel_motor,
            motor_r_vel: 0.0,
            motor_r_prio: 0,
        }
    }

    pub fn set_sensor(&mut self, value: String, prio: u8) {
        self.sensor = value;
        self.sensor_prio = prio;
    }

    pub fn set_motor_l(&mut self, value: f32, prio: u8) {
        let max_speed = 1.00;
        let _ =
            crate::hw_arduino::hw_motor_set_velocity(self.motor_l_pins, (value * max_speed).into());
        self.motor_l_vel = value;
        self.motor_l_prio = prio;
    }

    pub fn set_motor_r(&mut self, value: f32, prio: u8) {
        self.motor_r_vel = value;
        self.motor_r_prio = prio;
    }

    pub fn get_sensor(&mut self) -> (String, u8) {
        return (self.sensor.clone(), self.sensor_prio);
    }

    pub fn get_motor_l(&mut self) -> (f32, u8) {
        return (self.motor_l_vel, self.motor_l_prio);
    }

    pub fn get_motor_r(&mut self) -> (f32, u8) {
        return (self.motor_r_vel, self.motor_r_prio);
    }
}
