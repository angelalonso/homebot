use crate::action::CompositeAction as CAction;
use crate::input::Input;
#[cfg(any(feature = "test", feature = "live"))]
use crate::output_live::Output;
use crate::loggin::Log;
#[cfg(feature = "sim")]
use crate::output_sim::Output;

use std::time::Duration;

pub struct Brain {
    current: Vec<CAction>,
    incoming: Vec<CAction>,
    input: Input,
    output: Output,
    test_mode: bool,
}

impl Brain {
    pub async fn init(
        log: Log,
        test_mode: bool,
        time_step: i32,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let current = vec![];
        let incoming = vec![];
        let input = Input::init(time_step).await?;
        let output = Output::init(log.clone());
        Ok(Self {
            current,
            incoming,
            input,
            output,
            test_mode,
        })
    }

    pub fn update(&mut self, log: Log, add_incoming: String) -> Output {
        // We avoid doing this while testing, for higher control on tests
        // TODO: remove test_mode
        if !self.test_mode && add_incoming == "on" {
            //for ac in self.input.react(log.clone()) {
            //    self.add_incoming(ac);
            //}
        }
        let ts = self.input.update();
        let dist = self.input.get_distance();
        log.info(&format!(
            "--------------------- {:?} -- {:?} || {:?}|{:?}",
            ts,
            dist,
            self.output.get_motor_l(),
            self.output.get_motor_r()
        ));
        self.output.set_motor_l(1.00, 1);
        self.output.set_motor_r(-1.00, 1);
        //log.debug(&format!("iii: {:#?}", self.get_incoming_caction_ids()));

        // TODO: make this NOT A VECTOR
        // if dist[0] < 1250.0 {
        //     self.output.set_motor_l(1.00, 1);
        //     self.output.set_motor_r(-1.00, 1);
        // } else {
        //     self.output.set_motor_l(1.00, 1);
        //     self.output.set_motor_r(1.00, 1);
        // };
        // //// let mut tmp_incoming = vec![];
        // //// for i in self.incoming.iter_mut() {
        // ////     if i.starts_at <= ts.as_millis() {
        // ////         i.validate();
        // ////         self.current.push(i.clone());
        // ////     } else {
        // ////         tmp_incoming.push(i.clone());
        // ////     }
        // //// }
        // //// self.incoming = tmp_incoming;
        // //// let mut tmp_current = vec![];
        // //// // iterate through all CActions
        // //// for c in self.current.iter_mut() {
        // ////     let mut tmp_actions = vec![];
        // ////     // iterate through all Actions on that CAction
        // ////     for a in c.actions.iter_mut() {
        // ////         // if it has not yet "happened", leave it on current
        // ////         //   (add it to next version of current actually)
        // ////         if ts.as_millis() < (c.starts_at + a.delay_ms) {
        // ////             tmp_actions.push(a.clone())
        // ////         // if it is ongoing, leave it on current
        // ////         // (if no longer ongoing, have it removed)
        // ////         } else if ts.as_millis() < (c.starts_at + a.delay_ms + a.duration_ms) {
        // ////             tmp_actions.push(a.clone());
        // ////             // for each type of object, get it's current state
        // ////             // , then overwrite if the new one has a bigger prio
        // ////             if a.object == "sensor" {
        // ////                 let (_, s_p) = self.output.get_sensor(); // TODO: find a more elegant way
        // ////                 if s_p <= c.prio {
        // ////                     self.output.set_sensor(a.value.clone(), c.prio);
        // ////                 }
        // ////             } else if a.object == "motor_l" {
        // ////                 let (_, s_p) = self.output.get_motor_l(); // TODO: find a more elegant way
        // ////                 if s_p <= c.prio {
        // ////                     self.output.set_motor_l(
        // ////                         a.value
        // ////                             .parse::<f32>()
        // ////                             .expect("ERROR: value for motor_l was not a proper f32"),
        // ////                         c.prio,
        // ////                     );
        // ////                 }
        // ////             } else if a.object == "motor_r" {
        // ////                 let (_, s_p) = self.output.get_motor_r(); // TODO: find a more elegant way
        // ////                 if s_p <= c.prio {
        // ////                     self.output.set_motor_r(
        // ////                         a.value
        // ////                             .parse::<f32>()
        // ////                             .expect("ERROR: value for motor_r was not a proper f32"),
        // ////                         c.prio,
        // ////                     );
        // ////                 }
        // ////             }
        // ////         }
        // ////     }
        // ////     c.actions = tmp_actions.clone();
        // ////     if c.actions.len() > 0 {
        // ////         tmp_current.push(c.clone());
        // ////     }
        // //// }
        // //// self.current = tmp_current;
        self.output.clone()
    }

    pub fn set_testmode(&mut self, test: bool) {
        self.test_mode = test;
    }

    pub fn get_testmode(&mut self) -> bool {
        return self.test_mode.clone();
    }

    pub fn add_incoming(&mut self, c_action: CAction) {
        self.incoming.push(c_action);
    }

    pub fn get_incoming(&self) -> Vec<CAction> {
        return self.incoming.clone();
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
