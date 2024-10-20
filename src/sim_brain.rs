use std::time::Duration;

use crate::loggin::Log;
use crate::sim_action::Action;
use crate::sim_action::CompositeAction as CAction;
use crate::sim_queue::Queue;

use std::time::Duration;

pub struct Brain {
    current: Vec<CAction>,
    incoming: Vec<CAction>,
}

impl Brain {
    pub fn init() -> Self {
        let current = Queue::new();
        let incoming = Queue::new();
        Self { current, incoming }
    }

    pub fn add_incoming(&mut self, c_action: CAction) {
        self.incoming.add_c_action(c_action);
    }

    pub fn status_queues(&self, log: Log, tstamp: Duration) {
        log.debug(&format!(
            "{:#?} Incoming:{}",
            tstamp,
            self.incoming.actions_len()
        ));
    }

    pub fn get_current(&self) -> Queue {
        return self.current.clone();
    }

    pub fn get_incoming_action_ids(&self) -> Vec<String> {
        let mut result = vec![];
        for c in self.incoming.get_c_actions().iter() {
            for a in c.actions.iter() {
                result.push(a.get_id());
            }
        }
        result
    }

    pub fn update(&mut self, ts: Duration) {
        match &self
            .incoming
            .iter()
            .position(|x| ts.as_millis() > x.starts_at)
        {
            Some(i) => {
                for c in i.get_c_actions() {
                    if c.
                }
            }
            None => (),
        }
    }
}
