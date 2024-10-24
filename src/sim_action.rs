#[derive(Debug, Clone)]
pub struct Action {
    pub id: String,
    pub delay_ms: u128,
    pub duration_ms: u128,
    pub object: String,
    pub value: String,
}

impl Action {
    pub fn new(
        id: String,
        delay_ms: u128,
        duration_ms: u128,
        object: String,
        value: String,
    ) -> Self {
        Self {
            id,
            delay_ms,
            duration_ms,
            object,
            value,
        }
    }

    pub fn get_id(&self) -> String {
        return self.id.clone();
    }

    pub fn validate(&mut self) {
        /*
        This function just marks the id as "error_<id>" when the object is not valid
        , and as error_value_format when the value is not valid for a known object
        , this way we can continue our program without the wrong actions messing with it, but we keep the action object around.
         */
        match self.object.as_str() {
            // motor_r can be float, positive or negative
            "motor_r" => {
                let test_val: Result<f32, _> = self.value.parse();
                match test_val {
                    Ok(n) => (),
                    Err(_) => self.id = "error_value_format_".to_string() + &self.object,
                }
            }
            // motor_l can be float, positive or negative
            "motor_l" => {
                let test_val: Result<f32, _> = self.value.parse();
                match test_val {
                    Ok(n) => (),
                    Err(_) => self.id = "error_value_format_".to_string() + &self.object,
                }
            }
            // sensor can be "read" or "stop"
            "sensor" => match self.value.as_str() {
                "stop" | "read" => (),
                _ => self.id = "error_value_format_".to_string() + &self.object,
            },
            _ => self.id = "error_".to_string() + &self.id,
        }
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

    pub fn get_id(&self) -> String {
        return self.id.clone();
    }

    pub fn validate(&mut self) {
        for mut a in &mut self.actions {
            a.validate();
        }
    }
}

pub fn get_ids_from_act_array(array: Vec<Action>) -> Vec<String> {
    let mut id_array = vec![];
    for i in array {
        id_array.push(i.get_id());
    }
    id_array
}
