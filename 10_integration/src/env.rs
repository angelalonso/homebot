use std::collections::BTreeMap;
use std::time::SystemTime;

use crate::brain::Brain;
#[cfg(any(feature = "test", feature = "live"))]
use crate::hw::*;
#[cfg(any(feature = "test", feature = "live"))]
use crate::live_bindings::WbDeviceTag;
use crate::loggin::Log;
#[cfg(feature = "sim")]
use crate::sim_bindings::WbDeviceTag;
#[cfg(feature = "sim")]
use crate::sim_hw::*;

pub fn run(log: Log, cfg: BTreeMap<String, String>) -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(feature = "test")]
    let test_mode = true;
    #[cfg(any(feature = "sim", feature = "live"))]
    let test_mode = false;

    log.info("Checking config...");
    let time_step = cfg["TIME_STEP"].parse::<i32>()?;
    let _max_speed = cfg["MAX_SPEED"].parse::<f64>()?; // TODO: pass this to output
                                                       // -- Timestamps
    log.info("Configuring time...");
    let start_timestamp: SystemTime = SystemTime::now();
    // -- Init
    log.info("Loading bot, giving it a brain");
    robot_init();
    let mut brain = Brain::init(log.clone(), test_mode);
    // TODO: make distance sensors update the input constantly
    // TODO: send tstamp as input
    // -- LEDs - TODO: Add this part
    // -- Sensors
    log.info("Loading sensors...");
    let distance_sensor_names = vec!["distance_sensor_eyes"];
    // TODO: get sensors mockups for test
    // TODO: get sensors for live
    let mut distance_sensors: Vec<WbDeviceTag> = [].to_vec();
    distance_sensors = get_sensors_ids(distance_sensor_names, time_step);
    log.info("Running!");
    #[cfg(feature = "test")]
    let mut iteration = 0;
    loop {
        // Each iteration is marked by a timestamp
        let timestamp = start_timestamp
            .elapsed()
            .expect("Error retrieving time since start");
        let mut distance_values: Vec<f64> = [].to_vec();

        if robot_step(time_step) == -1 {
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
                .map(|sensor| distance_sensor_get_value(*sensor))
                .collect();
            brain.set_input_distance(log.clone(), distance_values);
        }
        let _active = brain.update(log.clone(), timestamp, sv);

        log.debug(&format!("---------------------"));
        //log.debug(&format!("{:#?}", timestamp));

        #[cfg(feature = "test")]
        {
            iteration += 1;
            if iteration >= 1 {
                break;
            }
        }
    }

    robot_cleanup();

    Ok(())
}
