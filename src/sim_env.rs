use crate::sim_bindings::WbDeviceTag;
use std::collections::BTreeMap;
use std::time::SystemTime;

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
    let mut distance_sensors: Vec<WbDeviceTag> = [].to_vec();
    if !test_mode {
        distance_sensors = distance_sensor_names
            .iter()
            .map(|name| {
                let sensor: WbDeviceTag = crate::wb_robot_get_device(name);
                crate::wb_distance_sensor_enable(sensor, time_step);
                sensor
            })
            .collect();
    }

    log.info("Running!");
    loop {
        // Each iteration is marked by a timestamp
        let timestamp = start_timestamp
            .elapsed()
            .expect("Error retrieving time since start");
        let mut distance_values: Vec<f64> = [].to_vec();
        // CAREFUL! This may be used to freeze time!!
        #[cfg(feature = "sim")]
        if crate::wb_robot_step(time_step) == -1 {
            break;
        }

        // TODO: move this to input code
        // TODO: check if related action allows for it
        //// Get values from sensors
        let (sv, _) = brain.get_output().get_sensor();
        if sv == "on" {
            if !test_mode {
                distance_values = distance_sensors
                    .iter()
                    .map(|sensor| crate::wb_distance_sensor_get_value(*sensor))
                    .collect();
            }
            brain.set_input_distance(log.clone(), distance_values);
        }
        let _active = brain.update(log.clone(), timestamp);

        log.debug(&format!("---------------------"));
        //log.debug(&format!("{:#?}", timestamp));
    }

    #[cfg(feature = "sim")]
    crate::wb_robot_cleanup();

    Ok(())
}
