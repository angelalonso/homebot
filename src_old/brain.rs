use crate::loggin::Log;
//use crate::queue::{Move, Queue};
use crate::actionqueue::CompositeAction;
use crate::actionqueue::Queue;

use std::time::Duration;

pub struct Brain {
    pub actions_queue: Queue,
}

impl Brain {
    pub fn init() -> Self {
        let actions_queue = Queue::new();
        //actions_queue.load_test();
        Self { actions_queue }
    }

    // TODO: create a new one with reading inputs as an action
    //pub fn refresh(&mut self, log: Log, now: Duration, distances: Vec<f64>) -> (f64, f64) {
    //    for d in distances.iter() {
    //        if *d < 1000. {
    //            let m1 = Move {
    //                left_speed: -1.,
    //                right_speed: 0.,
    //                millis: 1000,
    //                starts_at: 0,
    //                prio: true,
    //            };
    //            self.actions_queue.current = Some(m1);
    //        }
    //    }
    //    let (left_speed, right_speed) = self.actions_queue.update(now);
    //    self.status_current_moves(log.clone(), now);
    //    // TODO: choose what to do with several Moves, change this:
    //    //let mut l_s = 2.0;
    //    //let mut r_s = 2.0;

    //    //if distances[0] < 1500.0 {
    //    //    l_s = 0.0;
    //    //    r_s = -0.5;
    //    //};
    //    return (left_speed, right_speed);
    //}

    pub fn status_queues(&self, log: Log, tstamp: Duration) {
        log.debug(&format!(
            "{:#?} Incoming:{}",
            tstamp,
            self.actions_queue.incoming.len()
        ));
    }

    // TODO: add current, append current, and a timestamp for when it was added
    pub fn get_current(&self) -> Option<CompositeAction> {
        return self.actions_queue.current.clone();
    }

    pub fn get_incoming(&self) -> Vec<CompositeAction> {
        return self.actions_queue.incoming.clone();
    }

    pub fn status_current_moves(&self, log: Log, tstamp: Duration) {
        log.debug(&format!(
            "{:#?} Moves: {:#?}",
            tstamp, self.actions_queue.current
        ));
    }
}
