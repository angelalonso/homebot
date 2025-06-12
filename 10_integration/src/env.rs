use std::collections::BTreeMap;

use crate::brain::Brain;
#[cfg(any(feature = "test", feature = "live"))]
use crate::hw_arduino::*;
#[cfg(feature = "sim")]
use crate::hw_webots::*;
#[cfg(any(feature = "test", feature = "live"))]
use crate::live_output::Output;
use crate::loggin::Log;
#[cfg(feature = "sim")]
use crate::sim_output::Output;

pub async fn run(
    log: Log,
    cfg: BTreeMap<String, String>,
) -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(feature = "test")]
    let test_mode = true;
    #[cfg(any(feature = "sim", feature = "live"))]
    let test_mode = false;

    // -- Init
    let time_step = cfg["TIME_STEP"].parse::<i32>()?;
    let _max_speed = cfg["MAX_SPEED"].parse::<f64>()?; // TODO: pass this to output
    // it at robot_init() ??
    let mut b = Brain::init(log.clone(), test_mode, time_step).await?;
    let mut o = Output::init(log.clone());
    #[cfg(feature = "test")]
    let mut iteration = 0;
    // -- Loop
    // TODO: have only three components here: input, brain and output
    log.info("Running!");
    o.set_motor_l(1.00, 1);
    o.set_motor_r(1.00, 1);
    loop {
        if robot_step(time_step) == -1 {
            break;
        }
        let _ = b.update(log.clone(), "".to_string());

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
