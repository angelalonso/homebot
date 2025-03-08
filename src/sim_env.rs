use std::collections::BTreeMap;
use std::time::SystemTime;

use crate::homebot_brain::Brain;
use crate::loggin::Log;
use crate::sim_bindings::WbDeviceTag;
use crate::sim_hw::*;

pub fn run(log: Log, cfg: BTreeMap<String, String>) -> Result<(), Box<dyn std::error::Error>> {
    let test_mode = false;
    //
    //
    //
    log.info("Checking config...");
    let time_step = cfg["TIME_STEP"].parse::<i32>()?;
    let _max_speed = cfg["MAX_SPEED"].parse::<f64>()?; // TODO: pass this to output

    log.info("Configuring time...");
    let start_timestamp: SystemTime = SystemTime::now();

    log.info("Loading bot, giving it a brain");
    wb_robot_init();
    let mut brain = Brain::init(log.clone(), test_mode);
    // TODO: make distance sensors update the input constantly
    // TODO: send tstamp as input
    log.info("Loading sensors...");
    let distance_sensor_names = vec!["distance_sensor_eyes"];
    let mut distance_sensors: Vec<WbDeviceTag> = [].to_vec();
    distance_sensors = distance_sensor_names
        .iter()
        .map(|name| {
            let sensor: WbDeviceTag = crate::sim_hw::wb_robot_get_device(name);
            crate::sim_hw::wb_distance_sensor_enable(sensor, time_step);
            sensor
        })
        .collect();

    log.info("Running!");
    loop {
        // Each iteration is marked by a timestamp
        let timestamp = start_timestamp
            .elapsed()
            .expect("Error retrieving time since start");
        let mut distance_values: Vec<f64> = [].to_vec();
        // CAREFUL! This may be used to freeze time!!
        if crate::sim_hw::wb_robot_step(time_step) == -1 {
            break;
        }

        // TODO: move this to input code
        // TODO: check if related action allows for it
        //// Get values from sensors
        // TODO: avoid more incomings by a different tag than get_sensor
        // TODO: test with continuous distance
        let (sv, _) = brain.get_output().get_sensor();
        if sv == "on" {
            distance_values = distance_sensors
                .iter()
                .map(|sensor| crate::sim_hw::wb_distance_sensor_get_value(*sensor))
                .collect();
            brain.set_input_distance(log.clone(), distance_values);
        }
        let _active = brain.update(log.clone(), timestamp, sv);

        log.debug(&format!("---------------------"));
        //log.debug(&format!("{:#?}", timestamp));
        //
        //
        //
        //
        //
        //
        //
        //
    }

    #[cfg(feature = "sim")]
    crate::sim_hw::wb_robot_cleanup();

    Ok(())
}
