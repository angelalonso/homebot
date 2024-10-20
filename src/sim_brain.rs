use crate::loggin::Log;
use crate::sim_action::CompositeAction as CAction;
use crate::sim_queue::Queue;

use std::time::Duration;

pub struct Brain {
    current: Queue,
    incoming: Queue,
}

impl Brain {
    pub fn init() -> Self {
        let current = Queue::new();
        let incoming = Queue::new();
        Self { current, incoming }
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

    pub fn get_incoming(&self) -> Queue {
        return self.incoming.clone();
    }
}
