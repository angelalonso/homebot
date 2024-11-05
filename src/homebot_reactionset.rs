use crate::homebot_action::Action;
use crate::homebot_action::CompositeAction as CAction;
use std::time::Duration;

// TODO: accept variables but only optional (e.g.: id)
pub fn get_turnaround_reactionset(ts: Duration) -> CAction {
    // define some vars
    let id = "turn_around".to_string();
    let starts_at = ts.as_millis();

    // define actions
    let a1a = Action {
        id: "01_sensor_off".to_string(),
        delay_ms: 0,
        duration_ms: 1000,
        object: "sensor".to_string(),
        value: "off".to_string(),
    };
    let a1b = Action {
        id: "01_motor_l_back".to_string(),
        delay_ms: 0,
        duration_ms: 1000,
        object: "motor_l".to_string(),
        value: "-1.0".to_string(),
    };
    let a1c = Action {
        id: "01_motor_r_back".to_string(),
        delay_ms: 0,
        duration_ms: 1000,
        object: "motor_r".to_string(),
        value: "-1.0".to_string(),
    };
    let a2a = Action {
        id: "02_sensor_off".to_string(),
        delay_ms: 1000,
        duration_ms: 1000,
        object: "sensor".to_string(),
        value: "off".to_string(),
    };
    let a2b = Action {
        id: "02_motor_l_back".to_string(),
        delay_ms: 1000,
        duration_ms: 1000,
        object: "motor_l".to_string(),
        value: "-1.0".to_string(),
    };
    let a2c = Action {
        id: "02_motor_r_fwd".to_string(),
        delay_ms: 1000,
        duration_ms: 1000,
        object: "motor_r".to_string(),
        value: "1.0".to_string(),
    };
    let a3a = Action {
        id: "03_sensor_on".to_string(),
        delay_ms: 2000,
        duration_ms: 500,
        object: "sensor".to_string(),
        value: "on".to_string(),
    };
    let a3b = Action {
        id: "03_motor_l_stop".to_string(),
        delay_ms: 2000,
        duration_ms: 500,
        object: "motor_l".to_string(),
        value: "0.0".to_string(),
    };
    let a3c = Action {
        id: "03_motor_r_stop".to_string(),
        delay_ms: 2000,
        duration_ms: 500,
        object: "motor_r".to_string(),
        value: "0.0".to_string(),
    };
    // Create and fill the vector of actions
    let mut action_vector = vec![];
    action_vector.push(a1a);
    action_vector.push(a1b);
    action_vector.push(a1c);
    action_vector.push(a2a);
    action_vector.push(a2b);
    action_vector.push(a2c);
    action_vector.push(a3a);
    action_vector.push(a3b);
    action_vector.push(a3c);
    let turnaround_ca = CAction {
        id,
        actions: action_vector,
        starts_at,
        prio: 0,
    };

    return turnaround_ca;
}

pub fn get_moveon_reactionset(ts: Duration) -> CAction {
    // define some vars
    let id = "move_on".to_string();
    let starts_at = ts.as_millis();

    // define actions
    let a1a = Action {
        id: "01_sensor_on".to_string(),
        delay_ms: 0,
        duration_ms: 500,
        object: "sensor".to_string(),
        value: "on".to_string(),
    };
    let a1b = Action {
        id: "01_motor_l_fwd".to_string(),
        delay_ms: 0,
        duration_ms: 500,
        object: "motor_l".to_string(),
        value: "1.0".to_string(),
    };
    let a1c = Action {
        id: "01_motor_r_fwd".to_string(),
        delay_ms: 0,
        duration_ms: 500,
        object: "motor_r".to_string(),
        value: "1.0".to_string(),
    };
    // Create and fill the vector of actions
    let mut action_vector = vec![];
    action_vector.push(a1a);
    action_vector.push(a1b);
    action_vector.push(a1c);
    let moveon_ca = CAction {
        id,
        actions: action_vector,
        starts_at,
        prio: 0,
    };

    return moveon_ca;
}
