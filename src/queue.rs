use crate::movement::Move;

use std::time::Duration;

#[derive(Debug)]
pub struct Queue {
    pub now_actions: Vec<Move>,  // TODO: this should be all sorts of actions
    pub next_actions: Vec<Move>, // TODO: this should be all sorts of actions
}

impl Queue {
    pub fn new() -> Queue {
        Queue {
            now_actions: vec![],
            next_actions: vec![],
        }
    }

    pub fn add(&mut self, m: Move) {
        self.next_actions.push(m);
    }

    pub fn get_current(&mut self, ts: Duration) {
        match &self
            .next_actions
            .iter()
            .position(|x| ts.as_millis() > x.started_at)
        {
            Some(ix) => {
                self.now_actions.push(self.next_actions[*ix]);
                let _ = &self.next_actions.remove(*ix);
            }
            None => (),
        };
        match &self
            .now_actions
            .iter()
            .position(|x| ts.as_millis() > x.started_at + x.millis)
        {
            Some(ix) => {
                self.now_actions.remove(*ix);
            }
            None => (),
        };
    }

    pub fn list(&self) {
        println!("{:#?}", self.next_actions)
    }

    pub fn get_all(&self) -> Vec<Move> {
        self.next_actions.clone()
    }

    pub fn load_test(&mut self) {
        let m1 = Move {
            left_speed: 0.0,
            right_speed: -0.1,
            millis: 2000,
            started_at: 1000,
        };
        let m2 = Move {
            left_speed: 0.0,
            right_speed: -0.1,
            millis: 2000,
            started_at: 3000,
        };
        let m3 = Move {
            left_speed: 0.0,
            right_speed: -0.1,
            millis: 2000,
            started_at: 4000,
        };
        let m4 = Move {
            left_speed: 0.0,
            right_speed: -0.1,
            millis: 3000,
            started_at: 7000,
        };
        self.next_actions.push(m1);
        self.next_actions.push(m2);
        self.next_actions.push(m3);
        self.next_actions.push(m4);
    }
}
