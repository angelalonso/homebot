use crate::clibot::{distance_sensor_enable, Clibot, DeviceTag};
use std::collections::BTreeMap;
use std::time::SystemTime;

use crate::brain::Brain;
use crate::loggin::Log;

pub fn run(log: Log, cfg: BTreeMap<String, String>) -> Result<(), Box<dyn std::error::Error>> {
    log.info("Loading config...");
    let infinity = cfg["INFINITY"].parse::<f64>()?;
    let time_step = cfg["TIME_STEP"].parse::<i32>()?;
    let max_speed = cfg["MAX_SPEED"].parse::<f64>()?;

    log.info("Loading time and controller...");
    let start_timestamp: SystemTime = SystemTime::now();
    let mut brain = Brain::init();

    log.info("Loading bot and sensors...");
    let bot = Clibot::init("mike".to_string());
    let distance_sensor_names = vec!["distance_sensor_eyes"];
    let distance_sensors: Vec<DeviceTag> = distance_sensor_names
        .iter()
        .map(|name| {
            let sensor: DeviceTag = bot.get_device(name.to_string());
            distance_sensor_enable(log.clone(), sensor.clone(), time_step);
            sensor
        })
        .collect();

    log.info("Loading motors...");
    let mut left_motor = bot.get_device("left_wheel_motor".to_string());
    let mut right_motor = bot.get_device("right_wheel_motor".to_string());
    left_motor.set_position(infinity);
    right_motor.set_position(infinity);
    left_motor.set_velocity(0.1 * max_speed);
    right_motor.set_velocity(0.1 * max_speed);

    // TODO: replicate the simulate steps but for a real Robot
    log.info("Running!");
    loop {
        // Each iteration is marked by a timestamp
        let timestamp = start_timestamp
            .elapsed()
            .expect("Error retrieving time since start");
        if bot.step(time_step) == -1 {
            break;
        }

        // Get values from sensors
        let distance_values: Vec<f64> = distance_sensors
            .iter()
            .map(|sensor| sensor.get_sensor_value())
            .collect();

        // Define actions from sensor values
        let (left_speed, right_speed) =
            brain.refresh(log.clone(), timestamp, distance_values.clone());

        // write actuators inputs
        left_motor.set_velocity(left_speed);
        right_motor.set_velocity(right_speed);
    }

    bot.cleanup(log);
    Ok(())
}
