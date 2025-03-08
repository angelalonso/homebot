use homebot::homebot_aux_funcs::*;
use homebot::loggin::Log;

use std::collections::BTreeMap;
use std::io::Write;
use tempfile::NamedTempFile;

#[test]
fn test_check_cfg_missing_keys() {
    const CFGFILE: &str = "testcfg.yml";
    let mut data = BTreeMap::new();
    data.insert("DEV_LED1_PIN".to_string(), "17".to_string()); // Only one key provided
    let log = Log::init("DEBUG".to_string());
    check_cfg(data, CFGFILE, log); // Should log missing keys but not panic
}

#[test]
fn test_check_cfg_empty_values() {
    const CFGFILE: &str = "testcfg.yml";
    let mut data = BTreeMap::new();
    data.insert("DEV_LED1_PIN".to_string(), "".to_string()); // Empty value
    data.insert("DEV_LED2_PIN".to_string(), "18".to_string());
    let log = Log::init("DEBUG".to_string());
    check_cfg(data, CFGFILE, log); // Should log missing keys but not panic
}

#[test]
fn test_check_cfg_valid_data() {
    const CFGFILE: &str = "testcfg.yml";
    let mut data = BTreeMap::new();
    data.insert("DEV_LED1_PIN".to_string(), "17".to_string());
    data.insert("DEV_LED2_PIN".to_string(), "18".to_string());
    data.insert("DEV_MOTORL_PIN1".to_string(), "19".to_string());
    data.insert("DEV_MOTORL_PIN2".to_string(), "20".to_string());
    data.insert("DEV_MOTORL_PINE".to_string(), "21".to_string());
    data.insert("DEV_MOTORR_PIN1".to_string(), "22".to_string());
    data.insert("DEV_MOTORR_PIN2".to_string(), "23".to_string());
    data.insert("DEV_MOTORR_PINE".to_string(), "24".to_string());
    let log = Log::init("DEBUG".to_string());
    check_cfg(data, CFGFILE, log); // Should log missing keys but not panic
}

#[test]
fn test_load_valid_file() {
    let mut temp_file = NamedTempFile::new().expect("Failed to create temp file");
    writeln!(temp_file, "DEV_LED1_PIN: '17'\nDEV_LED2_PIN: '18'")
        .expect("Failed to write to temp file");
    let file_path = temp_file.path().to_str().unwrap();

    let result = load(file_path);
    assert!(result.is_ok(), "Expected successful loading of YAML");
    let data = result.unwrap();
    assert_eq!(data.get("DEV_LED1_PIN"), Some(&"17".to_string()));
    assert_eq!(data.get("DEV_LED2_PIN"), Some(&"18".to_string()));
}

#[test]
fn test_load_invalid_file() {
    let result = load("non_existent_file.yaml");
    assert!(result.is_err(), "Expected failure for non-existent file");
}
