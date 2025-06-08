use std::collections::BTreeMap;
use std::time::SystemTime;

use crate::brain::Brain;
use crate::hw::*;
use crate::loggin::Log;

pub fn run(log: Log, cfg: BTreeMap<String, String>) -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(feature = "test")]
    let test_mode = true;
    #[cfg(feature = "live")]
    let test_mode = false;
    log.info("Checking config...");
    let time_step = cfg["TIME_STEP"].parse::<i32>()?;
    let _max_speed = cfg["MAX_SPEED"].parse::<f64>()?; // TODO: pass this to output
                                                       // -- Timestamps
    log.info("Configuring time...");
    let start_timestamp: SystemTime = SystemTime::now();
    // -- Init
    log.info("Loading bot, giving it a brain");
    hw_robot_init();
    let mut brain = Brain::init(log.clone(), test_mode);
    // TODO: make distance sensors update the input constantly
    // TODO: send tstamp as input
    // -- LEDs
    log.info("Loading LEDs...");
    let _led_names = vec!["led001", "led002"];
    // -- Sensors
    log.info("Loading sensors...");
    let _distance_sensor_names = vec!["distance_sensor_eyes"];
    //let _distance_sensors: Vec<HWDeviceTag> = [].to_vec();
    // TODO: get sensors
    //
    //
    //
    //
    //
    //
    //
    //
    log.info("Running!");
    #[cfg(feature = "test")]
    let mut iteration = 0;
    loop {
        // Each iteration is marked by a timestamp
        let timestamp = start_timestamp
            .elapsed()
            .expect("Error retrieving time since start");
        let _distance_values: Vec<f64> = [].to_vec();
        // CAREFUL! This may be used to freeze time!!
        if hw_robot_step(time_step) == -1 {
            break;
        }
        //
        //
        //
        //
        //
        let (sv, _) = brain.get_output().get_sensor();
        //brain.set_input_distance(log.clone(), distance_values);
        //
        //
        //
        //
        //
        //
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

    #[cfg(feature = "sim")]
    crate::wb_robot_cleanup();

    Ok(())
}
