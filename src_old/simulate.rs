use crate::bindings::WbDeviceTag;
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
    crate::wb_robot_init();
    let distance_sensor_names = vec!["distance_sensor_eyes"];
    let distance_sensors: Vec<WbDeviceTag> = distance_sensor_names
        .iter()
        .map(|name| {
            let sensor: WbDeviceTag = crate::wb_robot_get_device(name);
            crate::wb_distance_sensor_enable(sensor, time_step);
            sensor
        })
        .collect();

    log.info("Loading motors...");
    let left_motor = crate::wb_robot_get_device("left_wheel_motor");
    let right_motor = crate::wb_robot_get_device("right_wheel_motor");
    crate::wb_motor_set_position(left_motor, infinity);
    crate::wb_motor_set_position(right_motor, infinity);
    crate::wb_motor_set_velocity(left_motor, 0.1 * max_speed);
    crate::wb_motor_set_velocity(right_motor, 0.1 * max_speed);

    log.info("Running!");
    loop {
        // Each iteration is marked by a timestamp
        let timestamp = start_timestamp
            .elapsed()
            .expect("Error retrieving time since start");
        if crate::wb_robot_step(time_step) == -1 {
            break;
        }

        // TODO:
        // Create queues for Input and Output,
        //   read them on refresh
        //   then read the sensors if needed,
        //   and actuate on the motors if needed.

        // OLD METHOD
        //// Get values from sensors
        //let distance_values: Vec<f64> = distance_sensors
        //    .iter()
        //    .map(|sensor| crate::wb_distance_sensor_get_value(*sensor))
        //    .collect();

        //// Define actions from sensor values
        //let (left_speed, right_speed) =
        //    brain.refresh(log.clone(), timestamp, distance_values.clone());

        //// write actuators inputs
        //crate::wb_motor_set_velocity(left_motor, left_speed);
        //crate::wb_motor_set_velocity(right_motor, right_speed);
        log.debug(&format!("{:#?}", timestamp));
    }

    crate::wb_robot_cleanup();
    Ok(())
}
