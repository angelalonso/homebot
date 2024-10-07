//use homebot::actionqueue::*;
//use homebot::brain::Brain;
//
//use std::collections::HashMap;
//use std::time::SystemTime;
//use std::{thread, time};
//
//// Action 2 overwrites action 1
//#[test]
//fn parallel_action() {
//    let start_timestamp: SystemTime = SystemTime::now();
//    let mut brain = Brain::init();
//
//    // Define time and expectation
//    let timelimit = 4.0;
//    let mut expected = HashMap::new();
//    expected.insert("0".to_string(), [""].to_vec());
//    expected.insert("1".to_string(), ["test", "test_2"].to_vec());
//    expected.insert("2".to_string(), ["test_22"].to_vec());
//    expected.insert("3".to_string(), [""].to_vec());
//
//    // Append the action(s)
//    let a1 = Action {
//        id: "test".to_string(),
//        starts_at: 1000,
//        millis: 500,
//        element: "".to_string(),
//    };
//    let a2 = Action {
//        id: "test_2".to_string(),
//        starts_at: 1000,
//        millis: 1500,
//        element: "".to_string(),
//    };
//    let mut comp_action_1 = vec![];
//    comp_action_1.push(a1);
//    comp_action_1.push(a2);
//    brain.actions_queue.add_incoming(&mut comp_action_1, 0);
//    // Run the tests
//    loop {
//        let timestamp = start_timestamp
//            .elapsed()
//            .expect("Error retrieving time since start");
//        if timestamp.as_secs_f32() >= timelimit {
//            break;
//        }
//        brain.actions_queue.update(timestamp);
//        //println!("- {:#?}", brain.get_current());
//        //println!("- {:#?}", brain.get_incoming());
//        //println!("------------------------------");
//        // TODO: check that action starts at second 1 and finishes at second 3
//        let ix = timestamp.as_secs_f32().floor() as i32;
//        //println!("{} -> {}", ix, expected[&ix.to_string()]);
//        let curr = &brain.get_current();
//        match curr {
//            Some(c) => {
//                let curr_act = &c.actions;
//                let mut current_ids = vec![];
//                for a in curr_act.iter() {
//                    current_ids.push(a.id.clone())
//                }
//                assert_eq!(current_ids, expected[&ix.to_string()]);
//            }
//            None => {
//                assert_eq!(vec![""], expected[&ix.to_string()]);
//            }
//        }
//        thread::sleep(time::Duration::from_secs(1));
//    }
//}
//// TODO: Action Composites:
//// Run X times
//// Repeat indefinitely
//// Run several actions in parallel
//// Run several actions in a sequence
