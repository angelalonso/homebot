use crate::bindings::WbDeviceTag;
use std::collections::BTreeMap;
use std::time::SystemTime;

use crate::loggin::Log;
use crate::sim_action::Action;
use crate::sim_brain::Brain;

pub fn run(log: Log, cfg: BTreeMap<String, String>) -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(feature = "test")]
    let test_mode = true;
    #[cfg(feature = "sim")]
    let test_mode = false;
    log.info("Loading config...");
    let infinity = cfg["INFINITY"].parse::<f64>()?;
    let time_step = cfg["TIME_STEP"].parse::<i32>()?;
    let max_speed = cfg["MAX_SPEED"].parse::<f64>()?;

    log.info("Configuring time...");
    let start_timestamp: SystemTime = SystemTime::now();

    log.info("Loading bot, giving it a brain");
    crate::wb_robot_init();
    let mut brain = Brain::init(test_mode);
    // TODO: make distance sensors update the input constantly
    // TODO: send tstamp as input
    log.info("Loading sensors...");
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
    //crate::wb_motor_set_velocity(left_motor, 0.5 * max_speed);
    //crate::wb_motor_set_velocity(right_motor, 0.5 * max_speed);
    crate::wb_motor_set_velocity(left_motor, 0.0);
    crate::wb_motor_set_velocity(right_motor, 0.0);

    log.info("Running!");
    loop {
        // Each iteration is marked by a timestamp
        let timestamp = start_timestamp
            .elapsed()
            .expect("Error retrieving time since start");
        // CAREFUL! This may be used to freeze time!!
        if crate::wb_robot_step(time_step) == -1 {
            break;
        }
        // TODO: move this to input code
        // TODO: check if related action allows for it
        //// Get values from sensors
        let distance_values: Vec<f64> = distance_sensors
            .iter()
            .map(|sensor| crate::wb_distance_sensor_get_value(*sensor))
            .collect();
        log.info(&format!("{:#?}", distance_values));
        // TODO:
        // pass distance and timestamp to input
        // let brain calculate and send us active actions
        // one by one, translate actions to...er...actions

        brain.get_input().set_distance(distance_values);
        let active = brain.update(timestamp);

        // TODO: move this to output code
        //// Define actions from sensor values
        //// write actuators inputs
        //crate::wb_motor_set_velocity(left_motor, left_speed);
        //crate::wb_motor_set_velocity(right_motor, right_speed);
        log.debug(&format!("{:#?}", timestamp));
    }

    crate::wb_robot_cleanup();
    Ok(())
}
