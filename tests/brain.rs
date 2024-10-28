use homebot::sim_action::get_ids_from_act_array;
use homebot::sim_action::Action;
use homebot::sim_action::CompositeAction as CAction;
use homebot::sim_brain::Brain;
use homebot::sim_output::Output;

use core::cmp::Ordering;
use std::collections::HashMap;
use std::time::SystemTime;
use std::{thread, time};

// TODO: simplify all this, make tests for each "process" that is done at brain instead, then maybe
// add extra tests for edge cases

#[test]
fn add_incoming() {
    /*
    This first test only checks that the Composite Actions (CAction) land on the incoming queue after having them added
    It does not validate them, so the id will remain the same
    */
    // Create the brain
    let mut brain = Brain::init(true);

    // Define expectation
    let expected = ["test_a"];

    // Append the action to a CAction
    let a1 = Action {
        id: "test_a".to_string(),
        delay_ms: 1000,
        duration_ms: 1500,
        object: "".to_string(),
        value: "".to_string(),
    };
    let mut action_vector = vec![];
    action_vector.push(a1);
    let c_action = CAction {
        id: "test_incoming".to_string(),
        actions: action_vector,
        starts_at: 0,
        prio: 0,
    };

    // add CAction to brain
    brain.add_incoming(c_action);

    // Run the tests
    //assert_eq!(brain.get_incoming_caction_ids(), expected);
    assert_eq!(brain.get_incoming_action_ids(), expected);
}

#[test]
fn incoming_to_current() {
    /*
    On this second test we check that the CActions are promoted from the incoming to the current queue as time progresses
    , then they are remove when all actions inside that CAction are done
    */
    // Create the brain and Time reference
    let mut brain = Brain::init(true);
    let start_timestamp: SystemTime = SystemTime::now();

    // Define time and expectation
    let timelimit = 4.0;
    let mut expected: HashMap<String, Vec<&str>> = HashMap::new();
    expected.insert("0".to_string(), [""].to_vec());
    expected.insert("1".to_string(), ["test_ca"].to_vec());
    expected.insert("2".to_string(), ["test_ca"].to_vec());
    expected.insert("3".to_string(), [""].to_vec());

    // Append the action to a CAction
    let a1 = Action {
        id: "test_a".to_string(),
        delay_ms: 500,
        duration_ms: 1000,
        object: "".to_string(),
        value: "".to_string(),
    };
    let mut action_vector = vec![];
    action_vector.push(a1);
    let c_action = CAction {
        id: "test_ca".to_string(),
        actions: action_vector,
        starts_at: 1000,
        prio: 0,
    };

    // add CAction to brain
    brain.add_incoming(c_action);

    // Run the tests
    loop {
        let timestamp = start_timestamp
            .elapsed()
            .expect("Error retrieving time since start");
        if timestamp.as_secs_f32() >= timelimit {
            break;
        }
        let _ = brain.update(timestamp);
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
fn incoming_to_current_validate() {
    /*
    On this second test we check that the CActions are promoted from the incoming to the current queue as time progresses
    , and we also check they are removed as they get done
    , but most importantly, they get their IDs transformed after validation.
    */
    // Create the brain and Time reference
    let mut brain = Brain::init(true);
    let start_timestamp: SystemTime = SystemTime::now();

    // Define time and expectation
    let timelimit = 4.0;
    let mut expected: HashMap<String, Output> = HashMap::new();
    let mut tmp_output: Output = Output::new();
    tmp_output.set_sensor("on".to_string(), 0);
    expected.insert("0".to_string(), tmp_output.clone());
    expected.insert("1".to_string(), tmp_output.clone());
    expected.insert("2".to_string(), tmp_output.clone());
    expected.insert("3".to_string(), tmp_output.clone());

    // Append the action to a CAction
    let a11 = Action {
        id: "test_a".to_string(),
        delay_ms: 500,
        duration_ms: 500,
        object: "".to_string(),
        value: "".to_string(),
    };
    let a12 = Action {
        id: "test_b".to_string(),
        delay_ms: 1000,
        duration_ms: 1500,
        object: "motor_r".to_string(),
        value: "".to_string(),
    };
    let mut action_vector_1 = vec![];
    action_vector_1.push(a11);
    action_vector_1.push(a12);
    let c_action_1 = CAction {
        id: "test_ca_1".to_string(),
        actions: action_vector_1,
        starts_at: 1000,
        prio: 0,
    };
    //
    let a21 = Action {
        id: "test_c".to_string(),
        delay_ms: 0,
        duration_ms: 1500,
        object: "motor_r".to_string(),
        value: "1".to_string(),
    };
    let mut action_vector_2 = vec![];
    action_vector_2.push(a21);
    let c_action_2 = CAction {
        id: "test_ca_2".to_string(),
        actions: action_vector_2,
        starts_at: 1000,
        prio: 0,
    };

    // add CAction to brain
    brain.add_incoming(c_action_1);
    brain.add_incoming(c_action_2);

    // Run the tests
    loop {
        let timestamp = start_timestamp
            .elapsed()
            .expect("Error retrieving time since start");
        if timestamp.as_secs_f32() >= timelimit {
            break;
        }
        let active = brain.update(timestamp);
        let ix = timestamp.as_secs_f32().floor() as i32;
        let curr = &brain.get_current_caction_ids();
        assert_eq!(
            format!("{:#?}", active),
            format!("{:#?}", expected[&ix.to_string()])
        );
        thread::sleep(time::Duration::from_secs(1));
    }
}

// TODO: do when previous works fine
//#[test]
//fn find_actives() {
//    /*
//    On this third test we check that the Actions in current are properly idenitified as active, and returned upon update of Brain.
//    */
//    // Create the brain and Time reference
//    let mut brain = Brain::init();
//    let start_timestamp: SystemTime = SystemTime::now();
//
//    // Define time and expectation
//    let timelimit = 4.0;
//    let mut expected: HashMap<String, Vec<&str>> = HashMap::new();
//    expected.insert(
//        "0".to_string(),
//        [
//            "error_value_format_sensor",
//            "01_motor_l_back",
//            "01_motor_r_back",
//        ]
//        .to_vec(),
//    );
//    expected.insert(
//        "1".to_string(),
//        [
//            "error_value_format_sensor",
//            "03_motor_l_stop",
//            "03_motor_r_stop",
//            "error_value_format_sensor",
//            "01_motor_l_back",
//            "01_motor_r_back",
//        ]
//        .to_vec(),
//    );
//    expected.insert(
//        "2".to_string(),
//        [
//            "test_a",
//            "error_value_format_sensor",
//            "03_motor_l_stop",
//            "03_motor_r_stop",
//            "error_value_format_sensor",
//            "01_motor_l_back",
//            "01_motor_r_back",
//        ]
//        .to_vec(),
//    );
//    expected.insert(
//        "3".to_string(),
//        [
//            "error_value_format_sensor",
//            "03_motor_l_stop",
//            "03_motor_r_stop",
//            "error_value_format_sensor",
//            "01_motor_l_back",
//            "01_motor_r_back",
//        ]
//        .to_vec(),
//    );
//
//    // Append the action to a CAction
//    let a1 = Action {
//        id: "test_a".to_string(),
//        delay_ms: 1000,
//        duration_ms: 500,
//        object: "motor_l".to_string(),
//        value: "0.5".to_string(),
//    };
//    let mut action_vector = vec![];
//    action_vector.push(a1);
//    let c_action = CAction {
//        id: "test_ca".to_string(),
//        actions: action_vector,
//        starts_at: 1000,
//        prio: 0,
//    };
//
//    // add CAction to brain
//    brain.add_incoming(c_action);
//
//    // Run the tests
//    loop {
//        let timestamp = start_timestamp
//            .elapsed()
//            .expect("Error retrieving time since start");
//        if timestamp.as_secs_f32() >= timelimit {
//            break;
//        }
//        let active = brain.update(timestamp);
//        let ix = timestamp.as_secs_f32().floor() as i32;
//        match active.len().cmp(&0) {
//            Ordering::Greater => {
//                assert_eq!(
//                    format!("{:#?}", get_ids_from_act_array(active)),
//                    format!("{:#?}", expected[&ix.to_string()])
//                );
//            }
//            _ => {
//                assert_eq!(vec![""], expected[&ix.to_string()]);
//            }
//        }
//        thread::sleep(time::Duration::from_secs(1));
//    }
//}
