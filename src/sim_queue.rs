use crate::sim_action::CompositeAction as CAction;

#[derive(Debug, Clone)]
pub struct Queue {
    c_actions: Vec<CAction>,
    delay_ms: u128,
    prio: u16,
}

impl Queue {
    pub fn new() -> Queue {
        Queue {
            c_actions: vec![],
            delay_ms: 0,
            prio: 0,
        }
    }

    pub fn actions_len(&self) -> usize {
        self.c_actions.len()
    }

    pub fn get_c_actions(&self) -> Vec<CAction> {
        self.c_actions.clone()
    }

    pub fn add_c_action(&mut self, c_action: CAction) {
        self.c_actions.push(c_action)
    }
}
