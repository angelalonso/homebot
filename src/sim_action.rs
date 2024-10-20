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
