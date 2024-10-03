use std::time::Duration;

#[derive(Debug, Clone, Copy)]
pub struct Action {
    pub finished: bool,
}

#[derive(Debug, Clone)]
pub struct CompositeAction {
    pub actions: Vec<Action>,
}

#[derive(Debug, Clone, Copy)]
pub struct Move {
    pub left_speed: f64,
    pub right_speed: f64,
    pub starts_at: u128,
    pub millis: u128,
    pub prio: bool, // true is now, false is next
}

#[derive(Debug, Clone)]
pub struct Queue {
    pub current: Option<Move>, // TODO: this should be all sorts of actions
    pub incoming: Vec<Move>,   // TODO: this should be all sorts of actions
}

impl Queue {
    pub fn new() -> Queue {
        Queue {
            current: None,
            incoming: vec![],
        }
    }

    pub fn add(&mut self, m: Move) {
        self.incoming.push(m);
    }

    pub fn update(&mut self, ts: Duration) -> (f64, f64) {
        self.update_cleandone(ts);
        let (left_speed, right_speed) = self.update_getnext(ts);
        return (left_speed, right_speed);
    }

    fn update_cleandone(&mut self, ts: Duration) {
        match self.current {
            None => {}
            Some(c) => {
                if c.millis > 0 {
                    // 0 or less means to run forever
                    if ts.as_millis() > c.starts_at + c.millis {
                        self.current = None;
                    }
                }
            }
        };
    }

    fn update_getnext(&mut self, ts: Duration) -> (f64, f64) {
        match &self
            .incoming
            .iter()
            .position(|x| ts.as_millis() > x.starts_at)
        {
            Some(ix) => {
                // If prio, get it on the current asap
                // If NOT prio:
                // - If current empty, make it current
                // - If current full, change it to start after current
                if self.incoming[*ix].prio == true {
                    self.current = Some(self.incoming[*ix]);
                    let _ = &self.incoming.remove(*ix);
                } else {
                    match self.current {
                        None => {
                            self.current = Some(self.incoming[*ix]);
                            let _ = &self.incoming.remove(*ix);
                        }
                        Some(c) => {
                            self.incoming[*ix].starts_at = c.starts_at + c.millis;
                        }
                    }
                }
            }
            None => (),
        };
        let result = match self.current {
            None => (0., 0.),
            Some(c) => (c.left_speed, c.right_speed),
        };
        return result;
    }

    pub fn list(&self) {
        println!("{:#?}", self.incoming)
    }

    pub fn get_all(&self) -> Vec<Move> {
        self.incoming.clone()
    }

    pub fn load_test(&mut self) {
        let m1 = Move {
            left_speed: 2.,
            right_speed: 2.,
            //millis: 2000,
            millis: 14000,
            starts_at: 1000,
            prio: true,
        };
        //let m2 = Move {
        //    left_speed: 0.,
        //    right_speed: -1.,
        //    millis: 2000,
        //    starts_at: 2000,
        //    prio: false,
        //};
        //let m3 = Move {
        //    left_speed: -1.,
        //    right_speed: 0.,
        //    millis: 2000,
        //    starts_at: 3000,
        //    prio: false,
        //};
        //let m4 = Move {
        //    left_speed: -2.,
        //    right_speed: -2.,
        //    millis: 0,
        //    starts_at: 4000,
        //    prio: false,
        //};
        self.incoming.push(m1);
        //self.incoming.push(m2);
        //self.incoming.push(m3);
        //self.incoming.push(m4);
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}
