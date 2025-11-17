use std::collections::BTreeMap;

pub fn load_cfg(filename: &str) -> Result<BTreeMap<String, String>, Box<dyn std::error::Error>> {
    let f = std::fs::File::open(filename)?;
    let dm: BTreeMap<String, String> = serde_yaml::from_reader(&f)?;
    Ok(dm)
}
