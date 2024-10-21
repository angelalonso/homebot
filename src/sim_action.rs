#[derive(Debug, Clone)]
pub struct Action {
    pub id: String,
    pub delay_ms: u128,
    pub duration_ms: u128,
    pub element: String,
}

impl Action {
    pub fn new(id: String, delay_ms: u128, duration_ms: u128, element: String) -> Action {
        Action {
            id,
            delay_ms,
            duration_ms,
            element,
        }
    }

    pub fn get_id(&self) -> String {
        return self.id.clone();
    }
}

#[derive(Debug, Clone)]
pub struct CompositeAction {
    pub id: String,
    pub actions: Vec<Action>,
    pub starts_at: u128,
    pub prio: u16,
}

impl CompositeAction {
    pub fn new(id: String, actions: Vec<Action>, starts_at: u128, prio: u16) -> CompositeAction {
        CompositeAction {
            id,
            actions,
            starts_at,
            prio,
        }
    }
}
