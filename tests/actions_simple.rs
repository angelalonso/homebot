//use homebot::sim_brain::Brain;
//use homebot::sim_queue::Queue;
//
//use std::collections::HashMap;
//use std::time::SystemTime;
//use std::{thread, time};
//
//// How it should work
//// we create actions,
////  add them to composite action.
////  send them to the brain,
////  then check when one or more actions are being executed.
//
//// Action that runs continuously for X secs
//#[test]
//fn simple_action_continuous_once() {
//    let start_timestamp: SystemTime = SystemTime::now();
//    let mut brain = Brain::init();
//
//    // Define time and expectation
//    let timelimit = 4.0;
//    let mut expected = HashMap::new();
//    expected.insert("0".to_string(), [""]);
//    expected.insert("1".to_string(), ["test_a"]);
//    expected.insert("2".to_string(), ["test_a"]);
//    expected.insert("3".to_string(), [""]);
//
//    // Append the action(s)
//    let a1 = Action {
//        id: "testaction".to_string(),
//        starts_at: 1000,
//        millis: 1500,
//        element: "".to_string(),
//    };
//    let mut c_action = vec![];
//    c_action.push(a1);
//    brain.actions_queue.add_incoming(&mut comp_action, 0);
//    // Run the tests
//    loop {
//        let timestamp = start_timestamp
//            .elapsed()
//            .expect("Error retrieving time since start");
//        if timestamp.as_secs_f32() >= timelimit {
//            break;
//        }
//        brain.actions_queue.update(timestamp);
//        let ix = timestamp.as_secs_f32().floor() as i32;
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

// // Action 2 overwrites action 1
// #[test]
// fn simple_action_priorities() {
// }
// TODO: Action Composites:
// Run X times
// Repeat indefinitely
// Run several actions in parallel
// Run several actions in a sequence
