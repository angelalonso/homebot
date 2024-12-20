//use homebot::sim_action::get_ids_from_act_array;
use homebot::homebot_action::Action;
use homebot::homebot_action::CompositeAction as CAction;
use homebot::homebot_brain::Brain;
use homebot::loggin::Log;
use homebot::test_output::Output;

use core::cmp::Ordering;
use std::collections::HashMap;
use std::time::SystemTime;
use std::{thread, time};

// TODO: simplify all this, make tests for each "process" that is done at brain instead, then maybe
// add extra tests for edge cases

#[test]
fn test_update_to_current() {
    /*
     * Test that given a set of CActions added to incoming,
     * they enter current when they should
     */
    // Create the brain and Time reference
    let log = Log::init("DEBUG".to_string());
    let mut brain = Brain::init(log.clone(), true);
    let start_timestamp: SystemTime = SystemTime::now();

    // Define time and expectation
    let timelimit = 4.0;
    let mut expected: HashMap<String, Vec<&str>> = HashMap::new();
    expected.insert("0".to_string(), ["test_ca1"].to_vec());
    expected.insert("1".to_string(), ["test_ca1", "test_ca2"].to_vec());
    expected.insert("2".to_string(), ["test_ca2"].to_vec());
    expected.insert("3".to_string(), [""].to_vec());

    // First action, First CAction
    let a1 = Action {
        id: "test_a".to_string(),
        delay_ms: 0,
        duration_ms: 1250,
        object: "".to_string(),
        value: "".to_string(),
    };
    let mut action_vector = vec![];
    action_vector.push(a1);
    let ca1 = CAction {
        id: "test_ca1".to_string(),
        actions: action_vector,
        starts_at: 0,
        prio: 0,
    };
    // Second action, Second CAction
    let a2 = Action {
        id: "test_b".to_string(),
        delay_ms: 250,
        duration_ms: 1500,
        object: "".to_string(),
        value: "".to_string(),
    };
    let mut action_vector_2 = vec![];
    action_vector_2.push(a2);
    let ca2 = CAction {
        id: "test_ca2".to_string(),
        actions: action_vector_2,
        starts_at: 500,
        prio: 0,
    };

    // add CActions to brain
    brain.add_incoming(ca1);
    brain.add_incoming(ca2);

    // Run the tests
    loop {
        let timestamp = start_timestamp
            .elapsed()
            .expect("Error retrieving time since start");
        if timestamp.as_secs_f32() >= timelimit {
            break;
        }
        let (sv, _) = brain.get_output().get_sensor();
        let _ = brain.update(log.clone(), timestamp, sv);
        let ix = timestamp.as_secs_f32().floor() as i32;
        let curr = &brain.get_current_caction_ids();
        match curr.len().cmp(&0) {
            Ordering::Greater => {
                assert_eq!(
                    format!("{:#?}", curr),
                    format!("{:#?}", expected[&ix.to_string()])
                );
            }
            _ => {
                assert_eq!(vec![""], expected[&ix.to_string()]);
            }
        }
        thread::sleep(time::Duration::from_secs(1));
    }
}

#[test]
fn test_update_to_output_straightaway() {
    /*
     * Test that given a set of CActions added to incoming,
     * they are sent to output when they should
     * TODO: another test for collision between actions on output
     * TODO: we need a default state to come back, maybe?
     */
    // Create the brain and Time reference
    let log = Log::init("DEBUG".to_string());
    let mut brain = Brain::init(log.clone(), true);
    let start_timestamp: SystemTime = SystemTime::now();

    // Define time and expectation
    let timelimit = 4.0;
    let mut expected: HashMap<String, Output> = HashMap::new();
    let mut this_output = Output::new(log.clone());
    expected.insert("0".to_string(), this_output.clone()); // the first composite action starts after 500ms
    this_output.set_sensor("on".to_string(), 0);
    this_output.set_motor_l(1.0, 0);
    this_output.set_motor_r(2.0, 0);
    expected.insert("1".to_string(), this_output.clone());
    expected.insert("2".to_string(), this_output.clone()); // the second starts at 2001ms so that it only shows up after this checkpoint
    this_output.set_sensor("off".to_string(), 0);
    this_output.set_motor_l(0.0, 0);
    this_output.set_motor_r(-1.0, 0);
    expected.insert("3".to_string(), this_output.clone());

    // First actions and CAction
    let a1 = Action {
        id: "a1_sensor".to_string(),
        delay_ms: 0,
        duration_ms: 1250,
        object: "sensor".to_string(),
        value: "on".to_string(),
    };
    let a2 = Action {
        id: "a2_motor_l".to_string(),
        delay_ms: 0,
        duration_ms: 1250,
        object: "motor_l".to_string(),
        value: "1.0".to_string(),
    };
    let a3 = Action {
        id: "a3_motor_r".to_string(),
        delay_ms: 0,
        duration_ms: 1250,
        object: "motor_r".to_string(),
        value: "2.0".to_string(),
    };
    let mut action_vector_1 = vec![];
    action_vector_1.push(a1);
    action_vector_1.push(a2);
    action_vector_1.push(a3);
    let ca1 = CAction {
        id: "test_ca1".to_string(),
        actions: action_vector_1,
        starts_at: 500,
        prio: 0,
    };
    // Second actions and CAction
    let b1 = Action {
        id: "b1_sensor".to_string(),
        delay_ms: 0,
        duration_ms: 1250,
        object: "sensor".to_string(),
        value: "off".to_string(),
    };
    let b2 = Action {
        id: "b2_motor_l".to_string(),
        delay_ms: 0,
        duration_ms: 1250,
        object: "motor_l".to_string(),
        value: "0.0".to_string(),
    };
    let b3 = Action {
        id: "b3_motor_r".to_string(),
        delay_ms: 0,
        duration_ms: 1250,
        object: "motor_r".to_string(),
        value: "-1.0".to_string(),
    };
    let mut action_vector_2 = vec![];
    action_vector_2.push(b1);
    action_vector_2.push(b2);
    action_vector_2.push(b3);
    let ca2 = CAction {
        id: "test_ca2".to_string(),
        actions: action_vector_2,
        starts_at: 2001,
        prio: 0,
    };

    // add CActions to brain
    brain.add_incoming(ca1);
    brain.add_incoming(ca2);

    // Run the tests
    loop {
        let timestamp = start_timestamp
            .elapsed()
            .expect("Error retrieving time since start");
        if timestamp.as_secs_f32() >= timelimit {
            break;
        }
        let (sv, _) = brain.get_output().get_sensor();
        let output = brain.update(log.clone(), timestamp, sv);
        let ix = timestamp.as_secs_f32().floor() as i32;
        assert_eq!(
            format!("{:#?}", output),
            format!("{:#?}", expected[&ix.to_string()])
        );
        thread::sleep(time::Duration::from_secs(1));
    }
}

#[test]
fn test_update_to_output_collisions() {
    /*
     * Test that given a set of CActions added to incoming,
     * they are sent to output when they should
     * AND their collisions are solved appropiately
     */
    // Create the brain and Time reference
    let log = Log::init("DEBUG".to_string());
    let mut brain = Brain::init(log.clone(), true);
    let start_timestamp: SystemTime = SystemTime::now();

    // Define time and expectation
    let timelimit = 4.0;
    let mut expected: HashMap<String, Output> = HashMap::new();
    let mut this_output = Output::new(log.clone());
    this_output.set_motor_l(10.0, 0);
    expected.insert("0".to_string(), this_output.clone()); // the first composite action starts after 500ms
    this_output.set_motor_l(0.0, 90);
    expected.insert("1".to_string(), this_output.clone());
    expected.insert("2".to_string(), this_output.clone()); // the second starts at 2001ms so that it only shows up after this checkpoint
    this_output.set_motor_l(2.0, 90);
    expected.insert("3".to_string(), this_output.clone());

    // Expected scenario:
    // b1 loads at 0, prio 0 -> 10.0
    // a1 loads at 500, prio 90, overwrites -> 0.0
    // b2 loads at 1500, prio 0 does not overwrite -> 0.0
    // a2 loads at 2500, prio 90, overwrite -> 2.0

    // First actions and CAction
    let a1 = Action {
        id: "a1_motor_l".to_string(),
        delay_ms: 500,
        duration_ms: 4000,
        object: "motor_l".to_string(),
        value: "0.0".to_string(),
    };
    let a2 = Action {
        id: "a2_motor_l".to_string(),
        delay_ms: 2500,
        duration_ms: 4000,
        object: "motor_l".to_string(),
        value: "2.0".to_string(),
    };
    let mut action_vector_1 = vec![];
    action_vector_1.push(a1);
    action_vector_1.push(a2);
    let ca1 = CAction {
        id: "test_ca1".to_string(),
        actions: action_vector_1,
        starts_at: 0,
        prio: 90,
    };
    // Second actions and CAction
    let b1 = Action {
        id: "b1_motor_l".to_string(),
        delay_ms: 0,
        duration_ms: 4000,
        object: "motor_l".to_string(),
        value: "10.0".to_string(),
    };
    let b2 = Action {
        id: "b2_motor_l".to_string(),
        delay_ms: 1500,
        duration_ms: 4000,
        object: "motor_l".to_string(),
        value: "20.0".to_string(),
    };
    let mut action_vector_2 = vec![];
    action_vector_2.push(b1);
    action_vector_2.push(b2);
    let ca2 = CAction {
        id: "test_ca2".to_string(),
        actions: action_vector_2,
        starts_at: 0,
        prio: 0,
    };

    // add CActions to brain
    brain.add_incoming(ca1);
    brain.add_incoming(ca2);

    // Run the tests
    loop {
        let timestamp = start_timestamp
            .elapsed()
            .expect("Error retrieving time since start");
        if timestamp.as_secs_f32() >= timelimit {
            break;
        }
        let (sv, _) = brain.get_output().get_sensor();
        let output = brain.update(log.clone(), timestamp, sv);
        let ix = timestamp.as_secs_f32().floor() as i32;
        assert_eq!(
            format!("{:#?}", output),
            format!("{:#?}", expected[&ix.to_string()])
        );
        thread::sleep(time::Duration::from_secs(1));
    }
}
