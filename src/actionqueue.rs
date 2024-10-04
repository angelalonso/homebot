use std::time::Duration;

#[derive(Debug, Clone)]
pub struct Action {
    pub id: String,
    pub starts_at: u128,
    pub millis: u128,
    pub element: String,
    pub prio: u16,
}

#[derive(Debug, Clone)]
pub struct CompositeAction {
    pub actions: Vec<Action>,
}

impl CompositeAction {
    pub fn new() -> CompositeAction {
        CompositeAction {
            actions: vec![],
        }
    }
}

#[derive(Debug, Clone)]
pub struct Queue {
    pub current: Vec<CompositeAction>,
    pub incoming: Vec<CompositeAction>,
}

impl Queue {
    pub fn new() -> Queue {
        Queue {
            current: vec![],
            incoming: vec![],
        }
    }

    pub fn add_incoming(&mut self, acts: &mut Vec<Action>) {
        let mut comp_action = CompositeAction::new();
        comp_action.actions.append(acts);
        self.incoming.push(comp_action);
    }

    pub fn update(&mut self, ts: Duration) -> String {
        self.update_cleandone(ts);
        let result = self.update_getnext(ts);
        return result;
    }

    fn update_cleandone(&mut self, ts: Duration) {
        if self.current.len() > 0 {
            if self.current[0].actions[0].millis > 0 {
            // 0 or less means to run forever
                if ts.as_millis() > self.current[0].actions[0].starts_at + self.current[0].actions[0].millis {
                    self.current.clear();
                }
            }
        };
    }

    fn update_getnext(&mut self, ts: Duration) -> String {
        match &self
            .incoming
            .iter()
            .position(|x| ts.as_millis() >= x.actions[0].starts_at)
        {
            Some(ix) => {
                // If prio, get it on the current asap
                // If NOT prio:
                // - If current empty, make it current
                // - If current full, change it to start after current
                if self.current.len() == 0 {
                    self.current = vec![self.incoming[*ix].clone()];
                    let _ = self.incoming.remove(*ix);
                } else if self.current[0].actions.len() == 0 {
                    self.current = vec![self.incoming[*ix].clone()];
                    let _ = self.incoming.remove(*ix);
                } else if self.incoming[*ix].actions[0].prio < self.current[0].actions[0].prio {
                    self.current = vec![self.incoming[*ix].clone()];
                    let _ = self.incoming.remove(*ix);
                } else {
                    // TODO: check if we overwrite or append
                    //   using something like:
                    //   self.incoming[*ix].actions[0].starts_at = c.actions[0].starts_at + c.actions[0].millis;
                    self.current = vec![self.incoming[*ix].clone()];
                    let _ = self.incoming.remove(*ix);
                }
            }
            None => (),
        };
        let result = match self.current.len() {
            0 => "".to_string(),
            _ => self.current[0].actions[0].id.clone(),
        };
        return result;
    }

}

