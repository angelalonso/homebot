use crate::bindings::WbDeviceTag;
use std::collections::BTreeMap;
use std::time::SystemTime;
use std::{thread, time::Duration};

use crate::loggin::Log;
use crate::sim_brain::Brain;

pub fn run(log: Log, cfg: BTreeMap<String, String>) -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(feature = "test")]
    let test_mode = true;
    #[cfg(feature = "sim")]
    let test_mode = false;
    log.info("Loading config...");
    let time_step = cfg["TIME_STEP"].parse::<i32>()?;
    let _max_speed = cfg["MAX_SPEED"].parse::<f64>()?; // TODO: pass this to output

    log.info("Configuring time...");
    let start_timestamp: SystemTime = SystemTime::now();

    log.info("Loading bot, giving it a brain");
    crate::wb_robot_init();
    let mut brain = Brain::init(log.clone(), test_mode);
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
        let (sensor_state, _) = brain.get_output().get_sensor();
        if sensor_state == "on" {
            thread::sleep(Duration::from_millis(900));
            let distance_values: Vec<f64> = distance_sensors
                .iter()
                .map(|sensor| crate::wb_distance_sensor_get_value(*sensor))
                .collect();
            log.info(&format!("{:#?}", distance_values));
            brain.get_input().set_distance(log.clone(), distance_values);
        } else {
            log.info(&format!("{:#?}", sensor_state));
        }
        let _active = brain.update(log.clone(), timestamp);

        //log.debug(&format!("{:#?}", timestamp));
    }

    crate::wb_robot_cleanup();
    Ok(())
}
