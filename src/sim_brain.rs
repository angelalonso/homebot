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

    pub fn update(&mut self, ts: Duration) -> Vec<Action> {
        let mut tmp_incoming = vec![];
        for i in self.incoming.iter_mut() {
            if i.starts_at <= ts.as_millis() {
                i.validate();
                self.current.push(i.clone());
            } else {
                tmp_incoming.push(i.clone());
            }
        }
        self.incoming = tmp_incoming;
        let mut result: Vec<Action> = vec![];
        let mut tmp_current = vec![];
        for c in self.current.iter_mut() {
            let mut tmp_actions = vec![];
            for a in c.actions.iter_mut() {
                if ts.as_millis() < (c.starts_at + a.delay_ms) {
                    tmp_actions.push(a.clone())
                } else if ts.as_millis() < (c.starts_at + a.delay_ms + a.duration_ms) {
                    tmp_actions.push(a.clone());
                    result.push(a.clone());
                }
            }
            c.actions = tmp_actions.clone();
            if c.actions.len() > 0 {
                tmp_current.push(c.clone());
            }
        }
        self.current = tmp_current;
        result
    }
}
