use homebot::homebot_input::Input;
use homebot::loggin::Log;
use std::time::Duration;

#[test]
fn test_input_new() {
    let mut input = Input::new(); // why mut?
    assert_eq!(input.get_ts(), Duration::from_millis(0));
    assert_eq!(input.get_distance(), vec![0.00]);
    assert_eq!(input.get_motor_l_pos(), 0.00);
    assert_eq!(input.get_motor_l_vel(), 0.00);
    assert_eq!(input.get_motor_r_pos(), 0.00);
    assert_eq!(input.get_motor_r_vel(), 0.00);
}

#[test]
fn test_input_set() {
    let mut input = Input::new();
    let ts = Duration::from_millis(100);
    let distance = vec![1000.0, 2000.0];
    input.set(ts, distance.clone(), 1.0, 2.0, 3.0, 4.0);

    assert_eq!(input.get_ts(), ts);
    assert_eq!(input.get_distance(), distance);
    assert_eq!(input.get_motor_l_pos(), 1.0);
    assert_eq!(input.get_motor_l_vel(), 2.0);
    assert_eq!(input.get_motor_r_pos(), 3.0);
    assert_eq!(input.get_motor_r_vel(), 4.0);
}

#[test]
fn test_input_set_ts() {
    let mut input = Input::new();
    let ts = Duration::from_secs(1);
    input.set_ts(ts);
    assert_eq!(input.get_ts(), ts);
}

#[test]
fn test_input_set_distance() {
    let mut input = Input::new();
    let log = Log::init("DEBUG".to_string());
    let distance = vec![500.0, 2500.0];
    input.set_distance(log, distance.clone());
    assert_eq!(input.get_distance(), distance);
}

#[test]
fn test_isit_turnaround() {
    let mut input = Input::new();
    let log = Log::init("DEBUG".to_string());
    input.set_distance(log, vec![1000.0, 2000.0]);
    let result = input.isit_turnaround();
    assert!(!result.is_empty());
}

#[test]
fn test_isit_moveon() {
    let mut input = Input::new();
    let log = Log::init("DEBUG".to_string());
    input.set_distance(log, vec![2000.0, 2500.0]);
    let result = input.isit_moveon();
    assert!(!result.is_empty());
}

#[test]
fn test_react() {
    let mut input = Input::new();
    input.set(Duration::from_secs(1), vec![1000.0], 0.0, 0.0, 0.0, 0.0);
    let log = Log::init("DEBUG".to_string());
    let result = input.react(log);
    assert!(!result.is_empty());
}
