use homebot::env::run;
use homebot::loggin::Log;
use std::collections::BTreeMap;

#[test]
fn test_run_valid_config() {
    let mut cfg = BTreeMap::new();
    cfg.insert("TIME_STEP".to_string(), "32".to_string());
    cfg.insert("MAX_SPEED".to_string(), "5.0".to_string());
    let log = Log::init("DEBUG".to_string());

    let result = run(log, cfg);
    assert!(result.is_ok(), "Expected run to execute successfully");
}

#[test]
fn test_run_invalid_time_step() {
    let mut cfg = BTreeMap::new();
    cfg.insert("TIME_STEP".to_string(), "invalid".to_string());
    cfg.insert("MAX_SPEED".to_string(), "5.0".to_string());
    let log = Log::init("DEBUG".to_string());

    let result = run(log, cfg);
    assert!(result.is_err(), "Expected failure due to invalid TIME_STEP");
}

#[test]
fn test_run_invalid_max_speed() {
    let mut cfg = BTreeMap::new();
    cfg.insert("TIME_STEP".to_string(), "32".to_string());
    cfg.insert("MAX_SPEED".to_string(), "invalid".to_string());
    let log = Log::init("DEBUG".to_string());

    let result = run(log, cfg);
    assert!(result.is_err(), "Expected failure due to invalid MAX_SPEED");
}
