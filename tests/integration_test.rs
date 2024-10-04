use homebot::actionqueue::*;
use homebot::brain::Brain;

use std::collections::HashMap;
use std::time::SystemTime;
use std::{thread, time};


// Action that runs continuously for X secs
#[test]
fn simple_action_continuous_once() {
    let start_timestamp: SystemTime = SystemTime::now();
    let mut brain = Brain::init();

    // Define time and expectation
    let timelimit = 4.0;
    let mut expected = HashMap::new();
    expected.insert( "0".to_string(), "".to_string() );
    expected.insert( "1".to_string(), "test".to_string() );
    expected.insert( "2".to_string(), "test".to_string() );
    expected.insert( "3".to_string(), "".to_string() );
    
    // Append the action(s)
    let a1 = Action {
        id: "test".to_string(),
        starts_at: 1000,
        millis: 1500,
        element: "".to_string(),
        prio: 0,
    };
    let mut comp_action = vec![];
    comp_action.push(a1);
    brain.actions_queue.add_incoming(&mut comp_action);
    // Run the tests
    loop {
        let timestamp = start_timestamp
            .elapsed()
            .expect("Error retrieving time since start");
        if timestamp.as_secs_f32() >= timelimit {
            break;
        }
        brain.actions_queue.update(timestamp);
        let ix = timestamp.as_secs_f32().floor() as i32;
        if brain.get_current().len() > 0 {
            assert_eq!(brain.get_current()[0].actions[0].id, expected[&ix.to_string()]);
        } else {
            assert_eq!("", expected[&ix.to_string()]);
        }
        thread::sleep(time::Duration::from_secs(1));
    }

}

// Action 2 overwrites action 1
#[test]
fn simple_action_priorities() {
    let start_timestamp: SystemTime = SystemTime::now();
    let mut brain = Brain::init();

    // Define time and expectation
    let timelimit = 4.0;
    let mut expected = HashMap::new();
    expected.insert( "0".to_string(), "".to_string() );
    expected.insert( "1".to_string(), "test".to_string() );
    expected.insert( "2".to_string(), "test_2".to_string() );
    expected.insert( "3".to_string(), "".to_string() );
    
    // Append the action(s)
    let a1 = Action {
        id: "test".to_string(),
        starts_at: 1000,
        millis: 1000,
        element: "".to_string(),
        prio: 1,
    };
    let a2 = Action {
        id: "test_2".to_string(),
        starts_at: 2000,
        millis: 500,
        element: "".to_string(),
        prio: 0,
    };
    let mut comp_action_1 = vec![];
    let mut comp_action_2 = vec![];
    comp_action_1.push(a1);
    comp_action_2.push(a2);
    brain.actions_queue.add_incoming(&mut comp_action_1);
    brain.actions_queue.add_incoming(&mut comp_action_2);
    // Run the tests
    loop {
        let timestamp = start_timestamp
            .elapsed()
            .expect("Error retrieving time since start");
        if timestamp.as_secs_f32() >= timelimit {
            break;
        }
        brain.actions_queue.update(timestamp);
        //println!("- {:#?}", brain.get_current());
        //println!("- {:#?}", brain.get_incoming());
        //println!("------------------------------");
        // TODO: check that action starts at second 1 and finishes at second 3
        let ix = timestamp.as_secs_f32().floor() as i32;
        //println!("{} -> {}", ix, expected[&ix.to_string()]);
        if brain.get_current().len() > 0 {
            assert_eq!(brain.get_current()[0].actions[0].id, expected[&ix.to_string()]);
        } else {
            assert_eq!("", expected[&ix.to_string()]);
        }
        thread::sleep(time::Duration::from_secs(1));
    }

}
    // TODO: Action Composites:
    // Run X times
    // Repeat indefinitely
    // Run several actions in parallel
    // Run several actions in a sequence
