use homebot::sim_action::Action;
use homebot::sim_action::CompositeAction as CAction;
use homebot::sim_brain::Brain;

use core::cmp::Ordering;
use std::collections::HashMap;
use std::time::SystemTime;
use std::{thread, time};

#[test]
fn add_incoming() {
    // Create the brain
    let mut brain = Brain::init();

    // Define expectation
    let expected = ["test_a"];

    // Append the action to a CAction
    let a1 = Action {
        id: "test_a".to_string(),
        delay_ms: 1000,
        duration_ms: 1500,
        element: "".to_string(),
    };
    let mut action_vector = vec![];
    action_vector.push(a1);
    let mut c_action = CAction {
        id: "test_ca".to_string(),
        actions: action_vector,
        starts_at: 0,
        prio: 0,
    };

    // add CAction to brain
    brain.add_incoming(c_action);

    // Run the tests
    assert_eq!(brain.get_incoming_action_ids(), expected);
}

#[test]
fn incoming_to_current() {
    // Create the brain and Time reference
    let mut brain = Brain::init();
    let start_timestamp: SystemTime = SystemTime::now();

    // Define time and expectation
    let timelimit = 4.0;
    let mut expected = HashMap::new();
    expected.insert("0".to_string(), [""]);
    expected.insert("1".to_string(), ["test_ca"]);
    expected.insert("2".to_string(), [""]);
    expected.insert("3".to_string(), [""]);

    // Append the action to a CAction
    let a1 = Action {
        id: "test_a".to_string(),
        delay_ms: 1000,
        duration_ms: 1500,
        element: "".to_string(),
    };
    let mut action_vector = vec![];
    action_vector.push(a1);
    let mut c_action = CAction {
        id: "test_ca".to_string(),
        actions: action_vector,
        starts_at: 0,
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
        brain.update(timestamp);
        let ix = timestamp.as_secs_f32().floor() as i32;
        let curr = &brain.get_current_cactions();
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
