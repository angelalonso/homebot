use homebot::homebot_reactionset::{get_moveon_reactionset, get_turnaround_reactionset};
use std::time::Duration;

#[test]
fn test_get_turnaround_reactionset() {
    let ts = Duration::from_secs(1);
    let ca = get_turnaround_reactionset(ts);
    assert_eq!(ca.id, "turn_around");
    assert_eq!(ca.starts_at, ts.as_millis());
    assert_eq!(ca.actions.len(), 9);
    assert_eq!(ca.actions[0].id, "01_sensor_off");
}

#[test]
fn test_get_moveon_reactionset() {
    let ts = Duration::from_secs(2);
    let ca = get_moveon_reactionset(ts);
    assert_eq!(ca.id, "move_on");
    assert_eq!(ca.starts_at, ts.as_millis());
    assert_eq!(ca.actions.len(), 3);
    assert_eq!(ca.actions[0].id, "01_sensor_on");
}
