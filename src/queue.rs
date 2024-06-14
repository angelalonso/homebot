use crate::movement::Move;

use std::time::Duration;

#[derive(Debug)]
pub struct Queue {
    pub actions: Vec<Move>, // TODO: this should be all sorts of actions
}

impl Queue {
    pub fn new() -> Queue {
        Queue { actions: vec![] }
    }

    pub fn add(&mut self, m: Move) {
        self.actions.push(m);
    }

    pub fn get_current(&mut self, ts: Duration, curr: Vec<Move>) -> Vec<Move> {
        let mut result: Vec<Move> = vec![];
        match &self
            .actions
            .iter()
            .position(|x| ts.as_millis() > x.started_at)
        {
            Some(ix) => {
                result.push(self.actions[*ix]);
                let _ = &self.actions.remove(*ix);
            }
            None => (),
        };
        match &curr
            .iter()
            .position(|x| ts.as_millis() < x.started_at + x.millis)
        {
            Some(ix) => {
                result.push(curr[*ix]);
            }
            None => (),
        };
        result
    }

    pub fn list(&self) {
        println!("{:#?}", self.actions)
    }

    pub fn get_all(&self) -> Vec<Move> {
        self.actions.clone()
    }

    pub fn load_test(&mut self) {
        let m1 = Move {
            left_speed: 0.0,
            right_speed: -0.1,
            millis: 2000,
            started_at: 1000,
        };
        //let m2 = Move {
        //    left_speed: 0.0,
        //    right_speed: -0.1,
        //    millis: 2000,
        //    started_at: 3000,
        //};
        //let m3 = Move {
        //    left_speed: 0.0,
        //    right_speed: -0.1,
        //    millis: 2000,
        //    started_at: 4000,
        //};
        //let m4 = Move {
        //    left_speed: 0.0,
        //    right_speed: -0.1,
        //    millis: 3000,
        //    started_at: 7000,
        //};
        self.actions.push(m1);
        //self.actions.push(m2);
        //self.actions.push(m3);
        //self.actions.push(m4);
    }
}
