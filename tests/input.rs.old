use std::time::Duration;

use homebot::sim_action::get_ids_from_cact_array as cact_ids;
use homebot::sim_action::Action;
use homebot::sim_action::CompositeAction as CAction;
use homebot::sim_input::Input;

#[test]
fn get_turnaround_cactions() {
    /*
    We update the inputs and try to get the set of CActions expected
     */
    let mut action_vector = vec![];
    let turnaround_ca = CAction {
        id: "turn_around".to_string(),
        actions: action_vector,
        starts_at: 0,
        prio: 0,
    };

    let mut expected: Vec<CAction> = [].to_vec();
    expected.push(turnaround_ca);

    let mut input = Input::new();
    // TODO: change only some
    input.set(
        Duration::from_millis(0),
        [15.0, 250.0].to_vec(),
        0.0,
        0.0,
        0.0,
        0.0,
    );
    let reaction = input.react();

    // Run the tests
    assert_eq!(cact_ids(reaction), cact_ids(expected));
}

#[test]
fn get_moveon_cactions() {
    /*
    We update the inputs and try to get the set of CActions expected
     */
    let mut action_vector = vec![];
    let moveon_ca = CAction {
        id: "move_on".to_string(),
        actions: action_vector,
        starts_at: 0,
        prio: 0,
    };

    let mut expected: Vec<CAction> = [].to_vec();
    expected.push(moveon_ca);

    let mut input = Input::new();
    // TODO: change only some
    input.set(
        Duration::from_millis(0),
        [120., 130.0, 220.0].to_vec(),
        0.0,
        0.0,
        0.0,
        0.0,
    );
    let reaction = input.react();

    // Run the tests
    assert_eq!(cact_ids(reaction), cact_ids(expected));
}
