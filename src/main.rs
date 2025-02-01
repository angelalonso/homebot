use homebot::loggin;

use crate::loggin::Log;
use serde_yaml;
use std::collections::BTreeMap;

const CFGFILE: &str = "cfg.yaml";

#[cfg(any(feature = "test", feature = "live"))]
use homebot::env::*;
#[cfg(feature = "sim")]
use homebot::sim_env::*;

pub fn check_cfg(data: BTreeMap<String, String>, log: Log) {
    let to_check = [
        "DEV_LED1_PIN",
        "DEV_LED2_PIN",
        "DEV_MOTORL_PIN1",
        "DEV_MOTORL_PIN2",
        "DEV_MOTORL_PINE",
        "DEV_MOTORR_PIN1",
        "DEV_MOTORR_PIN2",
        "DEV_MOTORR_PINE",
    ];
    for i in to_check {
        if !data.contains_key(&i.to_string()) {
            log.fatal(&format!("{} does not have {} defined!", CFGFILE, i));
        } else if data[i] == "" {
            log.err(&format!("{} is empty at {}!", i, CFGFILE));
        }
    }
    // TODO: exit if anything is missing
}

pub fn load(filename: &str) -> Result<BTreeMap<String, String>, Box<dyn std::error::Error>> {
    let f = std::fs::File::open(filename)?;
    let dm: BTreeMap<String, String> = serde_yaml::from_reader(&f)?;
    Ok(dm)
}

#[cfg(feature = "sim")]
fn main() {
    match load(CFGFILE) {
        Ok(cfg) => {
            let log = loggin::Log::init(cfg["LOGLEVEL"].clone());
            log.info(&format!("- Mode: Webots Simulation"));
            match run(log.clone(), cfg) {
                Ok(()) => (),
                Err(es) => {
                    log.err(&format!("ERROR running simulation: {:#?}", es));
                }
            };
        }
        Err(e) => {
            let log = loggin::Log::init("DEBUG".to_string());
            log.err(&format!("ERROR Reading YAML: {:#?}", e));
        }
    };
}

#[cfg(feature = "test")]
fn main() {
    match load(CFGFILE) {
        Ok(cfg) => {
            let log = loggin::Log::init(cfg["LOGLEVEL"].clone());
            check_cfg(cfg.clone(), log.clone());
            log.info(&format!("- Mode: Code Tests"));
            match run(log.clone(), cfg) {
                Ok(()) => (),
                Err(es) => {
                    log.err(&format!("ERROR running tests: {:#?}", es));
                }
            };
        }
        Err(e) => {
            let log = loggin::Log::init("DEBUG".to_string());
            log.err(&format!("ERROR Reading YAML: {:#?}", e));
        }
    };
}

#[cfg(feature = "live")]
fn main() {
    match load(CFGFILE) {
        Ok(cfg) => {
            let log = loggin::Log::init(cfg["LOGLEVEL"].clone());
            check_cfg(cfg.clone(), log.clone());
            log.info(&format!("- Mode: Hardware Live"));
            //match run(log.clone(), cfg) {
            //    Ok(()) => (),
            //    Err(es) => {
            //        log.err(&format!("ERROR running live: {:#?}", es));
            //    }
            //};
        }
        Err(e) => {
            let log = loggin::Log::init("DEBUG".to_string());
            log.err(&format!("ERROR Reading YAML: {:#?}", e));
        }
    };
}
