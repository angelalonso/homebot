use crate::loggin::Log;
use crate::sim_action::Action;
use crate::sim_action::CompositeAction as CAction;
use crate::sim_input::Input;
use crate::sim_output::Output;

use std::time::Duration;

pub struct Brain {
    current: Vec<CAction>,
    incoming: Vec<CAction>,
    input: Input,
    output: Output,
    test_mode: bool,
}

impl Brain {
    pub fn init(test_mode: bool) -> Self {
        let current = vec![];
        let incoming = vec![];
        let input = Input::new();
        let output = Output::new();
        Self {
            current,
            incoming,
            input,
            output,
            test_mode,
        }
    }

    pub fn update(&mut self, ts: Duration) -> Output {
        self.input.set_ts(ts);
        // We avoid doing this while testing, for higher control on tests
        if !self.test_mode {
            for ac in self.input.react() {
                self.add_incoming(ac);
            }
        }
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
        let mut tmp_current = vec![];
        for c in self.current.iter_mut() {
            let mut tmp_actions = vec![];
            for a in c.actions.iter_mut() {
                if ts.as_millis() < (c.starts_at + a.delay_ms) {
                    tmp_actions.push(a.clone())
                } else if ts.as_millis() < (c.starts_at + a.delay_ms + a.duration_ms) {
                    tmp_actions.push(a.clone());
                    //
                    //
                    if a.object == "sensor" {
                        let (_, s_p) = self.output.get_sensor(); // TODO: find a more elegant way
                        if s_p <= c.prio {
                            self.output.set_sensor(a.value.clone(), c.prio);
                        }
                    }
                }
            }
            c.actions = tmp_actions.clone();
            if c.actions.len() > 0 {
                tmp_current.push(c.clone());
            }
        }
        self.current = tmp_current;
        self.output.clone()
    }

    pub fn set_testmode(&mut self, test: bool) {
        self.test_mode = test;
    }

    pub fn get_input(&self) -> Input {
        return self.input.clone();
    }

    pub fn add_incoming(&mut self, c_action: CAction) {
        self.incoming.push(c_action);
    }

    pub fn get_incoming_caction_ids(&self) -> Vec<String> {
        let mut result = vec![];
        for c in self.incoming.iter() {
            result.push(c.get_id());
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

    pub fn status_queues(&self, log: Log, tstamp: Duration) {
        log.debug(&format!("{:#?} Incoming:{}", tstamp, self.incoming.len()));
    }

    pub fn get_current(&self) -> Vec<CAction> {
        return self.current.clone();
    }

    pub fn get_current_caction_ids(&self) -> Vec<String> {
        let mut result = vec![];
        for i in self.current.clone() {
            result.push(i.id)
        }
        result
    }

    pub fn get_current_action_ids(&self) -> Vec<String> {
        let mut result = vec![];
        for c in self.current.clone() {
            for a in c.actions.iter() {
                result.push(a.get_id());
            }
        }
        result
    }

    pub fn get_output(&self) -> Output {
        return self.output.clone();
    }
}
