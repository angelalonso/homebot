use homebot::sim_action::Action;
use homebot::sim_action::CompositeAction as CAction;
use homebot::sim_brain::Brain;

#[test]
fn create_caction_with_validated_action() {
    /*
     TODO: maybe we get to validate actions from here, or maybe from the brain itself?
    */
    // Create the brain
    let mut brain = Brain::init();

    // Define expectation
    let expected = ["motor_fwd", "error_other_stuff"];

    // Create actions and add them to CAction
    let mut test = Action {
        id: "motor_fwd".to_string(),
        delay_ms: 0,
        duration_ms: 0,
        object: "motor_r".to_string(),
        value: "1.00".to_string(),
    };
    test.validate();
    let mut test2 = Action {
        id: "other_stuff".to_string(),
        delay_ms: 0,
        duration_ms: 0,
        object: "some_object".to_string(),
        value: "1.00".to_string(),
    };
    test2.validate();
    let mut action_vector = vec![];
    action_vector.push(test);
    action_vector.push(test2);
    let c_action = CAction {
        id: "test_ca".to_string(),
        actions: action_vector,
        starts_at: 1000,
        prio: 0,
    };

    // add CAction to brain
    brain.add_incoming(c_action);

    // Run the tests
    assert_eq!(brain.get_incoming_action_ids(), expected);
}

#[test]
fn parse_action() {
    /*
    Here we simply test the validate function, which just marks the id as "error_<id>",
    */
    let mut a1 = Action {
        id: "motor_r_fwd".to_string(),
        delay_ms: 0,
        duration_ms: 0,
        object: "motor_r".to_string(),
        value: "1.00".to_string(),
    };
    a1.validate();
    assert_eq!(a1.get_id(), "motor_r_fwd");
    //
    let mut a2 = Action {
        id: "motor_l_fwd".to_string(),
        delay_ms: 0,
        duration_ms: 0,
        object: "motor_l".to_string(),
        value: "1".to_string(),
    };
    a2.validate();
    assert_eq!(a2.get_id(), "motor_l_fwd");
    //
    let mut a3 = Action {
        id: "motor_r_fwd".to_string(),
        delay_ms: 0,
        duration_ms: 0,
        object: "motor_r".to_string(),
        value: "0,1,".to_string(),
    };
    a3.validate();
    assert_eq!(a3.get_id(), "error_value_format_motor_r");
    //
    let mut a4 = Action {
        id: "sensor_read".to_string(),
        delay_ms: 0,
        duration_ms: 0,
        object: "sensor".to_string(),
        value: "stop".to_string(),
    };
    a4.validate();
    assert_eq!(a4.get_id(), "sensor_read");
    //
    let mut a5 = Action {
        id: "sensor_read".to_string(),
        delay_ms: 0,
        duration_ms: 0,
        object: "sensor".to_string(),
        value: "readx".to_string(),
    };
    a5.validate();
    assert_eq!(a5.get_id(), "error_value_format_sensor");
}
