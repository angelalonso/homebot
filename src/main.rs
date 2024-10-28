use homebot::loggin;

use serde_yaml;
use std::collections::BTreeMap;

#[cfg(any(feature = "sim", feature = "test"))]
use homebot::sim;

pub fn load(filename: &str) -> Result<BTreeMap<String, String>, Box<dyn std::error::Error>> {
    let f = std::fs::File::open(filename)?;
    let dm: BTreeMap<String, String> = serde_yaml::from_reader(&f)?;
    Ok(dm)
}

#[cfg(any(feature = "sim", feature = "test"))]
fn main() {
    match load("cfg.yaml") {
        Ok(cfg) => {
            let log = loggin::Log::init(cfg["LOGLEVEL"].clone());
            log.info(&format!("- Mode: Webots Simulation"));
            match sim::run(log.clone(), cfg) {
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
// TODO:
// #[cfg(feature = "live")]
// fn main() {
// ...
