use homebot::homebot_aux_funcs::*;
use homebot::loggin;
use std::fs;

const CFGFILE: &str = "cfg.yaml";

#[cfg(any(feature = "test", feature = "live"))]
use homebot::env::*;
#[cfg(feature = "sim")]
use homebot::sim_env::*;

fn main() {
    println!("Running...");

    // Check if the config file exists
    if fs::metadata(CFGFILE).is_err() {
        let log = loggin::Log::init("DEBUG".to_string());
        log.err(&format!("ERROR: Config file '{}' does not exist.", CFGFILE));
        println!("Create one. Bye!");
        return;
    }

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
            match run(log.clone(), cfg.clone()) {
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
}
