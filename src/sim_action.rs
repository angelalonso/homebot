#[derive(Debug, Clone)]
pub struct Action {
    pub id: String,
    pub starts_at: u128,
    pub millis: u128,
    pub element: String,
}

impl Action {
    pub fn new(id: String, starts_at: u128, millis: u128, element: String) -> Action {
        Action {
            id,
            starts_at,
            millis,
            element,
        }
    }

    pub fn get_id(&self) -> String {
        return self.id.clone();
    }
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
