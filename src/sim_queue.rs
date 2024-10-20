use crate::sim_action::CompositeAction as CAction;

#[derive(Debug, Clone)]
pub struct Queue {
    actions: Vec<CAction>,
    delay_ms: u128,
    prio: u16,
}

impl Queue {
    pub fn new() -> Queue {
        Queue {
            actions: vec![],
            delay_ms: 0,
            prio: 0,
        }
    }

    pub fn actions_len(&self) -> usize {
        self.actions.len()
    }
}
