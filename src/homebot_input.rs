use crate::homebot_action::CompositeAction as CAction;
use crate::homebot_reactionset::get_moveon_reactionset;
use crate::homebot_reactionset::get_turnaround_reactionset;
use crate::loggin::Log;

use std::time::Duration;

#[derive(Debug, Clone)]
pub struct Input {
    ts: Duration,
    distance: Vec<f64>,
    motor_l_pos: f32,
    motor_l_vel: f32,
    motor_r_pos: f32,
    motor_r_vel: f32,
}

impl Input {
    pub fn new() -> Self {
        Self {
            ts: Duration::from_millis(0),
            distance: [0.00].to_vec(),
            motor_l_pos: 0.00,
            motor_l_vel: 0.00,
            motor_r_pos: 0.00,
            motor_r_vel: 0.00,
        }
    }

    pub fn set(
        &mut self,
        ts: Duration,
        distance: Vec<f64>,
        motor_l_pos: f32,
        motor_l_vel: f32,
        motor_r_pos: f32,
        motor_r_vel: f32,
    ) {
        self.ts = ts;
        self.distance = distance;
        self.motor_l_pos = motor_l_pos;
        self.motor_l_vel = motor_l_vel;
        self.motor_r_pos = motor_r_pos;
        self.motor_r_vel = motor_r_vel;
    }

    pub fn set_ts(&mut self, ts: Duration) {
        self.ts = ts;
    }

    pub fn set_distance(&mut self, _log: Log, distance: Vec<f64>) {
        self.distance = distance.clone();
    }

    pub fn react(&self, log: Log) -> Vec<CAction> {
        let mut result = vec![];
        let ta = self.isit_turnaround();
        let mo = self.isit_moveon();
        log.debug(&format!("----{:#?}", self.distance[0]));
        log.debug(&format!("TA: {:#?}, MO: {:#?}", ta.len(), mo.len()));
        for ca in ta {
            log.info(&format!("-TA"));
            result.push(ca);
        }
        for ca in mo {
            log.info(&format!("-MO"));
            result.push(ca);
        }
        result
    }

    pub fn isit_turnaround(&self) -> Vec<CAction> {
        let mut yes_itis = false;
        let mut result = vec![];
        for dist in &self.distance {
            if *dist < 1500.0 {
                yes_itis = true;
            };
        }
        if yes_itis {
            result.push(get_turnaround_reactionset(self.ts.clone()));
        };
        return result;
    }

    pub fn isit_moveon(&self) -> Vec<CAction> {
        let mut yes_itis = true;
        let mut result = vec![];
        for dist in &self.distance {
            if *dist < 1500.0 {
                yes_itis = false;
            };
        }
        if yes_itis {
            result.push(get_moveon_reactionset(self.ts.clone()));
        };
        return result;
    }
}
