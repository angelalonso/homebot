use std::time::Duration;

#[derive(Debug)]
pub struct Move {
    left_speed: f64,
    right_speed: f64,
    millis: i64,
    started_at: Duration,
}

pub fn get_speed(distance_sensors: Vec<f64>) -> (f64, f64) {
    let mut l_s = 0.5;
    let mut r_s = 0.5;

    if distance_sensors[0] < 1500.0 {
        l_s = 0.0;
        r_s = -0.1;
    };
    return (l_s, r_s);
}

pub fn get(distance_sensors: Vec<f64>, ts: Duration) -> Option<Move> {
    if distance_sensors[0] < 1500.0 {
        let m = Move {
            left_speed: 0.0,
            right_speed: -0.1,
            millis: 1000,
            started_at: ts,
        };
        return Some(m);
    } else {
        return None;
    }
}
