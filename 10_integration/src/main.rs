use std::collections::BTreeMap;
use std::fs;

use homebot::aux_funcs::*;
use homebot::brain::Brain;
#[cfg(any(feature = "test", feature = "live"))]
use homebot::hw_live::*;
#[cfg(feature = "sim")]
use homebot::hw_sim::*;
use homebot::loggin;

pub async fn run(
    log: loggin::Log,
    cfg: BTreeMap<String, String>,
) -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(feature = "test")]
    let test_mode = true;
    #[cfg(any(feature = "sim", feature = "live"))]
    let test_mode = false;
    // -- Init
    let time_step = cfg["TIME_STEP"].parse::<i32>()?;
    let _max_speed = cfg["MAX_SPEED"].parse::<f64>()?; // TODO: pass this to output
                                                       // TODO: function to check and put
                                                       // together motor vars
    #[cfg(feature = "test")]
    let mut iteration = 0;
    let mut b = Brain::init(log.clone(), cfg.clone(), test_mode, time_step).await?;
    // -- Loop
    log.info("Running!");
    loop {
        if robot_step(time_step) == -1 {
            break;
        }
        let _ = b.update(log.clone(), "".to_string()).await;
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

const CFGFILE: &str = "cfg.yml";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Running...");
    // Check if the config file exists
    if fs::metadata(CFGFILE).is_err() {
        let log = loggin::Log::init("DEBUG".to_string());
        log.err(&format!("ERROR: Config file '{}' does not exist.", CFGFILE));
        println!(
            "Create a {} config file before running, please. Bye!",
            CFGFILE
        );
        ()
    }
    // load and modify some variables
    match load(CFGFILE) {
        Ok(mut cfg) => {
            #[cfg(feature = "test")]
            cfg.insert("MODE".to_string(), "Code Testing".to_string());
            #[cfg(feature = "sim")]
            cfg.insert("MODE".to_string(), "Webots Simulation".to_string());
            #[cfg(feature = "live")]
            cfg.insert("MODE".to_string(), "real Hardware Run".to_string());
            let log = loggin::Log::init(cfg["LOGLEVEL"].clone());
            log.info(&format!("- Mode: {}", cfg["MODE"]));
            // Make the Robot Run
            match run(log.clone(), cfg.clone()).await {
                Ok(()) => (),
                Err(es) => {
                    log.err(&format!("ERROR while on {}: {:#?}", cfg["MODE"], es));
                }
            };
        }
        Err(e) => {
            let log = loggin::Log::init("DEBUG".to_string());
            log.err(&format!("ERROR Reading YAML: {:#?}", e));
        }
    };
    println!("Bye!");
    Ok(())
}
