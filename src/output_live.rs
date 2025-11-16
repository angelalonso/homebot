use gpio_cdev::{Chip, LineHandle, LineRequestFlags};
use std::collections::BTreeMap;
use std::error::Error;
use std::sync::{Arc, Mutex};

// use crate::hw_live::*;
use crate::loggin::Log;

#[derive(Debug, Clone)]
pub struct Output {
    sensor: String,
    sensor_prio: u8,
    motor_l: MotorCtrl,
    motor_r: MotorCtrl,
    //    #[allow(dead_code)]
    //    motor_l_pins: (u32, u32, u32),
    //    motor_l_vel: f32,
    motor_l_prio: u8,
    //    #[allow(dead_code)]
    //    motor_r_pins: (u32, u32, u32),
    //    motor_r_vel: f32,
    motor_r_prio: u8,
}

impl Output {
    pub fn init(log: Log, cfg: BTreeMap<String, String>) -> Self {
        //let left_wheel_motor: (u32, u32, u32);
        //let right_wheel_motor: (u32, u32, u32);
        //// TODO: pass it from config
        //let MOTOR_A_ENABLE: u32 = 22;
        //let MOTOR_A_IN1: u32 = 17;
        //let MOTOR_A_IN2: u32 = 27;
        //let MOTOR_B_ENABLE: u32 = 25;
        //let MOTOR_B_IN1: u32 = 23;
        //let MOTOR_B_IN2: u32 = 24;
        let motor_l = MotorCtrl::new(
            cfg["MOTORL_PINE"].parse::<u32>().unwrap_or_else(|_| {
                log.err("ISSUE at Motor L ENA");
                0
            }),
            cfg["MOTORL_PIN1"].parse::<u32>().unwrap_or_else(|_| {
                log.err("ISSUE at Motor L 1");
                0
            }),
            cfg["MOTORL_PIN2"].parse::<u32>().unwrap_or_else(|_| {
                log.err("ISSUE at Motor L 2");
                0
            }),
        )
        .expect("ERROR LOADING LEFT MOTOR");

        let motor_r = MotorCtrl::new(
            cfg["MOTORR_PINE"].parse::<u32>().unwrap_or_else(|_| {
                log.err("ISSUE at Motor R ENA");
                0
            }),
            cfg["MOTORR_PIN1"].parse::<u32>().unwrap_or_else(|_| {
                log.err("ISSUE at Motor R 1");
                0
            }),
            cfg["MOTORR_PIN2"].parse::<u32>().unwrap_or_else(|_| {
                log.err("ISSUE at Motor R 2");
                0
            }),
        )
        .expect("ERROR LOADING RIGHT MOTOR");

        let _infinity = f64::INFINITY;
        //left_wheel_motor = (17, 27, 22);
        //right_wheel_motor = (23, 24, 25);

        Self {
            sensor: "on".to_string(),
            sensor_prio: 0,
            motor_l,
            motor_r,
            //motor_l_pins: left_wheel_motor,
            //motor_l_vel: 0.0,
            motor_l_prio: 0,
            //motor_r_pins: right_wheel_motor,
            //motor_r_vel: 0.0,
            motor_r_prio: 0,
        }
    }

    pub fn set_sensor(&mut self, value: String, prio: u8) {
        self.sensor = value;
        self.sensor_prio = prio;
    }

    pub fn set_motor_l(&mut self, value: f32, prio: u8) {
        let factor = 1.0;
        let speed_factored = (value * factor) as i32;
        let _ = self.motor_l.set_speed(speed_factored as i32);

        self.motor_l_prio = prio;
    }

    pub fn set_motor_r(&mut self, value: f32, prio: u8) {
        let factor = 1.0;
        let speed_factored = (value * factor) as i32;
        let _ = self.motor_r.set_speed(speed_factored as i32);
        self.motor_r_prio = prio;
    }

    pub fn get_sensor(&mut self) -> (String, u8) {
        return (self.sensor.clone(), self.sensor_prio);
    }

    pub fn get_motor_l_prio(&mut self) -> u8 {
        return self.motor_l_prio;
    }

    pub fn get_motor_r_prio(&mut self) -> u8 {
        return self.motor_r_prio;
    }
}

#[derive(Debug, Clone)]
struct MotorCtrl {
    enable: Arc<Mutex<LineHandle>>,
    in1: Arc<Mutex<LineHandle>>,
    in2: Arc<Mutex<LineHandle>>,
}

impl MotorCtrl {
    fn new(enable_pin: u32, in1_pin: u32, in2_pin: u32) -> Result<Self, Box<dyn Error>> {
        let mut chip = Chip::new("/dev/gpiochip0")?;

        // Request lines with output direction
        let enable = chip
            .get_line(enable_pin)?
            .request(LineRequestFlags::OUTPUT, 0, "MOTOR_EN")?;
        let in1 = chip
            .get_line(in1_pin)?
            .request(LineRequestFlags::OUTPUT, 0, "MOTOR_IN1")?;
        let in2 = chip
            .get_line(in2_pin)?
            .request(LineRequestFlags::OUTPUT, 0, "MOTOR_IN2")?;

        Ok(Self {
            enable: Arc::new(Mutex::new(enable)),
            in1: Arc::new(Mutex::new(in1)),
            in2: Arc::new(Mutex::new(in2)),
        })
    }

    fn set_speed(&mut self, speed: i32) -> Result<(), Box<dyn Error>> {
        let enable = self.enable.lock().unwrap();
        let in1 = self.in1.lock().unwrap();
        let in2 = self.in2.lock().unwrap();
        if speed > 0 {
            // Forward
            in1.set_value(1)?;
            in2.set_value(0)?;
            // Simulate PWM by toggling (crude implementation)
            enable.set_value(1)?;
        } else if speed < 0 {
            // Reverse
            in1.set_value(0)?;
            in2.set_value(1)?;
            // Simulate PWM by toggling
            enable.set_value(1)?;
        } else {
            // Stop
            in1.set_value(0)?;
            in2.set_value(0)?;
            enable.set_value(0)?;
        }
        Ok(())
    }
}
