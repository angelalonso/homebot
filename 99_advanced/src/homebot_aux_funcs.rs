use crate::loggin::Log;

use std::collections::BTreeMap;

pub fn check_cfg(data: BTreeMap<String, String>, cfgfile: &str, log: Log) {
    let to_check = [
        "DEV_LED1_PIN",
        "DEV_MOTORL_PIN1",
        "DEV_MOTORL_PIN2",
        "DEV_MOTORL_PINE",
        "DEV_MOTORR_PIN1",
        "DEV_MOTORR_PIN2",
        "DEV_MOTORR_PINE",
    ];
    for i in to_check {
        if !data.contains_key(&i.to_string()) {
            log.fatal(&format!("{} does not have {} defined!", cfgfile, i));
        } else if data[i] == "" {
            log.err(&format!("{} is empty at {}!", i, cfgfile));
        }
    }
    // TODO: exit if anything is missing
}

pub fn load(filename: &str) -> Result<BTreeMap<String, String>, Box<dyn std::error::Error>> {
    let f = std::fs::File::open(filename)?;
    let dm: BTreeMap<String, String> = serde_yaml::from_reader(&f)?;
    Ok(dm)
}
