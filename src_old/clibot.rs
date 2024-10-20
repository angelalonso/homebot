use crate::loggin::Log;

#[derive(Clone)]
pub struct DeviceTag {
    pub name: String,
    pub x: f64,
    pub y: f64,
    pub v_x: f64,
    pub v_y: f64,
    pub a_x: f64,
    pub a_y: f64,
    pub value: f64,
}

impl DeviceTag {
    // wb_robot_init
    pub fn init(name: String) -> Self {
        Self {
            name,
            x: 0.,
            y: 0.,
            v_x: 0.,
            v_y: 1.,
            a_x: 0.,
            a_y: 0.,
            value: 0.,
        }
    }

    pub fn set_position(&mut self, position: f64) {
        self.x = position;
        self.y = position;
    }

    pub fn set_velocity(&mut self, velocity: f64) {
        self.v_x = velocity;
        self.v_y = velocity;
    }

    pub fn get_sensor_value(&self) -> f64 {
        self.value
    }
}

pub struct Clibot {
    pub name: String,
    pub devices: Vec<DeviceTag>,
    pub x: f64,
    pub y: f64,
    pub v_x: f64,
    pub v_y: f64,
    pub a_x: f64,
    pub a_y: f64,
}

impl Clibot {
    // wb_robot_init
    pub fn init(name: String) -> Self {
        let dist_sensor = DeviceTag::init("distance_sensor_eyes".to_string());
        let lw_motor = DeviceTag::init("left_wheel_motor".to_string());
        let rw_motor = DeviceTag::init("right_wheel_motor".to_string());
        let devices = [dist_sensor, lw_motor, rw_motor].to_vec();

        Self {
            name,
            devices,
            x: 0.,
            y: 0.,
            v_x: 0.,
            v_y: 1.,
            a_x: 0.,
            a_y: 0.,
        }
    }

    pub fn step(&self, _step: i32) -> i32 {
        return 0;
    }

    pub fn cleanup(&self, log: Log) {
        log.info("Clean up Bot");
    }

    pub fn get_device(&self, name: String) -> DeviceTag {
        for i in &self.devices {
            if i.name == name {
                return i.clone();
            }
        }
        let dummy = DeviceTag::init("dummy".to_string());
        dummy
    }

    pub fn set_position(&mut self, position: f64) {
        self.x = position;
        self.y = position;
    }

    pub fn set_velocity(&mut self, velocity: f64) {
        self.v_x = velocity;
        self.v_y = velocity;
    }

    pub fn update_pos(&mut self) -> (f64, f64) {
        self.v_x = self.v_x + self.a_x;
        self.v_y = self.v_y + self.a_y;
        self.x = self.x + self.v_x;
        self.y = self.y + self.v_y;
        return (self.x, self.y);
    }

    pub fn get_pos(&self) -> (f64, f64) {
        return (self.x, self.y);
    }

    pub fn change_v(&mut self, v_x: f64, v_y: f64) {
        self.v_x = v_x;
        self.v_y = v_y;
    }

    pub fn change_a(&mut self, a_x: f64, a_y: f64) {
        self.a_x = a_x;
        self.a_y = a_y;
    }
}

pub fn distance_sensor_enable(log: Log, sensor: DeviceTag, _timestep: i32) {
    log.info(&format!("Sensor {} enabled", sensor.name));
}
