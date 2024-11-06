use std::time::Duration;

#[derive(Debug, Clone)]
pub struct Action {
    pub id: String,
    pub starts_at: u128,
    pub millis: u128,
    pub element: String,
}

#[derive(Debug, Clone)]
pub struct CompositeAction {
    pub actions: Vec<Action>,
    pub delay_ms: u128,
    pub prio: u16,
}

impl CompositeAction {
    pub fn new() -> CompositeAction {
        CompositeAction {
            actions: vec![],
            delay_ms: 0,
            prio: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Queue {
    pub current: Option<CompositeAction>,
    pub current_started_at: u128,
    pub incoming: Vec<CompositeAction>,
}

impl Queue {
    pub fn new() -> Queue {
        Queue {
            current: None,
            current_started_at: 0,
            incoming: vec![],
        }
    }

    pub fn add_incoming(&mut self, acts: &mut Vec<Action>, prio: u16) {
        let mut comp_action = CompositeAction::new();
        comp_action.prio = prio;
        comp_action.actions.append(acts);
        self.incoming.push(comp_action);
    }

    pub fn update(&mut self, ts: Duration) -> String {
        self.update_cleandone(ts);
        let result = self.update_getnext(ts);
        return result;
    }

    fn update_cleandone(&mut self, ts: Duration) {
        match &mut self.current {
            Some(c) => {
                let del = c.delay_ms;
                c.actions
                    .retain(|a| a.millis > 0 && ts.as_millis() > a.starts_at + a.millis + del)
            }
            None => (),
        };
    }

    fn update_getnext(&mut self, ts: Duration) -> String {
        match &self
            .incoming
            .iter()
            .position(|x| ts.as_millis() >= x.delay_ms)
        {
            Some(ix) => {
                // If prio, get it on the current asap
                // If NOT prio:
                // - If current empty, make it current
                // - If current full, change it to start after current
                // ----------------------
                match &self.current {
                    None => {
                        self.current = Some(self.incoming[*ix].clone());
                        let _ = self.incoming.remove(*ix);
                    }
                    Some(c) => {
                        if self.incoming[*ix].prio < c.prio {
                            self.current = Some(self.incoming[*ix].clone());
                            let _ = self.incoming.remove(*ix);
                        } else {
                            // TODO: check if we overwrite or append
                            //   using something like:
                            //   self.incoming[*ix].actions[0].starts_at = c.actions[0].starts_at + c.actions[0].millis;
                            self.current = Some(self.incoming[*ix].clone());
                            let _ = self.incoming.remove(*ix);
                        }
                    }
                }
            }
            None => (),
        };
        let result = match &self.current {
            None => "".to_string(),
            Some(c) => {
                let mut tmp = "".to_string();
                for a in c.actions.iter() {
                    tmp = tmp.to_owned() + ", " + &a.id;
                }
                tmp.to_string()
            }
        };
        return result;
    }
}
