use homebot::loggin;

use serde_yaml;
use std::collections::BTreeMap;

#[cfg(feature = "simulate")]
use homebot::simulate;

#[cfg(feature = "clisimulate")]
use homebot::clisimulate;

pub fn load(filename: &str) -> Result<BTreeMap<String, String>, Box<dyn std::error::Error>> {
    let f = std::fs::File::open(filename)?;
    let dm: BTreeMap<String, String> = serde_yaml::from_reader(&f)?;
    Ok(dm)
}

#[cfg(feature = "simulate")]
fn main() {
    match load("cfg.yaml") {
        Ok(cfg) => {
            let log = loggin::Log::init(cfg["LOGLEVEL"].clone());
            log.info(&format!("- Mode: Webots Simulation"));
            match simulate::run(log.clone(), cfg) {
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

#[cfg(feature = "clisimulate")]
fn main() {
    match load("cfg.yaml") {
        Ok(cfg) => {
            let log = loggin::Log::init(cfg["LOGLEVEL"].clone());
            log.info(&format!("- Mode: CLI Simulation"));
            match clisimulate::run(log, cfg) {
                Ok(()) => (),
                Err(ec) => {
                    log.err(&format!("ERROR running CLI simulation: {:#?}", ec));
                }
            };
        }
        Err(e) => {
            let log = loggin::Log::init("DEBUG".to_string());
            log.err(&format!("ERROR Reading YAML: {:#?}", e));
        }
    };
}

// TODO: fill this up
#[cfg(feature = "live")]
fn main() {
    match load("cfg.yaml") {
        Ok(cfg) => {
            let log = loggin::Log::init(cfg["LOGLEVEL"].clone());
            log.info(&format!("- Mode: Live Run"));
        }
        Err(e) => {
            let log = loggin::Log::init("DEBUG".to_string());
            log.err(&format!("ERROR Reading YAML: {:#?}", e));
        }
    };
}

#[cfg(not(feature = "simulate"))]
#[cfg(not(feature = "clisimulate"))]
#[cfg(not(feature = "live"))]
fn main() {
    match load("cfg.yaml") {
        Ok(cfg) => {
            let log = loggin::Log::init(cfg["LOGLEVEL"].clone());
            log.err(&format!(
                "- Mode Unknown, please choose simulate, clisimulate or live"
            ));
        }
        Err(e) => {
            log.err(&format!("ERROR Reading YAML: {:#?}", e));
        }
    };
}
