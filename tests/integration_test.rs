use homebot::actionqueue::*;
use homebot::brain::Brain;

use std::collections::HashMap;
use std::time::SystemTime;
use std::{thread, time};


#[test]
fn simple_action() {
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
    brain.actions_queue.add_incoming(a1);
    // Run the tests
    loop {
        let timestamp = start_timestamp
            .elapsed()
            .expect("Error retrieving time since start");
        if timestamp.as_secs_f32() >= timelimit {
            break;
        }
        brain.actions_queue.update(timestamp);
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
    // Run once
    // Run continuously for X secs
    // Run X times
    // Repeat indefenitely
    // Run several actions in parallel
    // Run several actions in a sequence
