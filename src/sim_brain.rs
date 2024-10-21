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
        let current = vec![];
        let incoming = vec![];
        Self { current, incoming }
    }

    pub fn add_incoming(&mut self, c_action: CAction) {
        self.incoming.push(c_action);
    }

    pub fn status_queues(&self, log: Log, tstamp: Duration) {
        log.debug(&format!("{:#?} Incoming:{}", tstamp, self.incoming.len()));
    }

    pub fn get_current(&self) -> Vec<CAction> {
        return self.current.clone();
    }

    pub fn get_current_cactions(&self) -> Vec<String> {
        let mut result = vec![];
        for i in self.current.clone() {
            result.push(i.id)
        }
        result
    }

    pub fn get_incoming_action_ids(&self) -> Vec<String> {
        let mut result = vec![];
        for c in self.incoming.iter() {
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
                //for c in i.get_c_actions() {
                //    if c.
                //}
                ()
            }
            None => (),
        }
    }
}
