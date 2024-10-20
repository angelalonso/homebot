use homebot::sim_action::Action;
use homebot::sim_action::CompositeAction as CAction;
use homebot::sim_brain::Brain;

#[test]
fn add_incoming() {
    // Create the brain
    let mut brain = Brain::init();

    // Define expectation
    let expected = ["test_a"];

    // Append the action to a CAction
    let a1 = Action {
        id: "test_a".to_string(),
        starts_at: 1000,
        millis: 1500,
        element: "".to_string(),
    };
    let mut action_vector = vec![];
    action_vector.push(a1);
    let mut c_action = CAction {
        actions: action_vector,
        delay_ms: 0,
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
    expected.insert("1".to_string(), ["test_a"]);
    expected.insert("2".to_string(), ["test_a"]);
    expected.insert("3".to_string(), [""]);

    // Append the action to a CAction
    let a1 = Action {
        id: "test_a".to_string(),
        starts_at: 1000,
        millis: 1500,
        element: "".to_string(),
    };
    let mut action_vector = vec![];
    action_vector.push(a1);
    let mut c_action = CAction {
        actions: action_vector,
        delay_ms: 0,
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
        brain.actions_queue.update(timestamp);
        let ix = timestamp.as_secs_f32().floor() as i32;
        let curr = &brain.get_current();
        match curr {
            Some(c) => {
                let curr_act = &c.actions;
                let mut current_ids = vec![];
                for a in curr_act.iter() {
                    current_ids.push(a.id.clone())
                }
                assert_eq!(current_ids, expected[&ix.to_string()]);
            }
            None => {
                assert_eq!(vec![""], expected[&ix.to_string()]);
            }
        }
        thread::sleep(time::Duration::from_secs(1));
    }
}
