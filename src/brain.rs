use std::collections::{BTreeMap, VecDeque, HashMap};
use std::time::{Duration, Instant};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;

use crate::action::CompositeAction as CAction;
use crate::action::Action;
use crate::input::Input;
use crate::loggin::Log;
#[cfg(any(feature = "test", feature = "live"))]
use crate::output_live::Output;
#[cfg(feature = "sim")]
use crate::output_sim::Output;


#[cfg(feature = "test")]
#[derive(Clone, Debug)]
pub struct MockOutput {
    pub motors: Vec<(String, f32)>,
    pub leds: Vec<bool>,
    pub logs: Vec<String>,
}

#[cfg(feature = "test")]
impl MockOutput {
    pub fn new() -> Self {
        Self {
            motors: vec![],
            leds: vec![],
            logs: vec![],
        }
    }

    pub fn set_motor(&mut self, motor: &str, value: f32) {
        self.motors.push((motor.to_string(), value));
    }

    pub fn set_led(&mut self, state: bool) {
        self.leds.push(state);
    }

    pub fn log(&mut self, msg: &str) {
        self.logs.push(msg.to_string());
    }
}


// Condition system
#[derive(Debug, Clone)]
pub enum Condition {
    DistanceLessThan(f32),
    DistanceGreaterThan(f32),
    TimeSinceLastAction(Duration),
    ButtonPressed,
    EncoderReached(i32),
    And(Box<Condition>, Box<Condition>),
    Or(Box<Condition>, Box<Condition>),
    Not(Box<Condition>),
    Always,
}

// Rename to RuleAction to avoid conflict with existing Action
#[derive(Debug, Clone)]
pub enum RuleActionType {
    MoveLeftWheel(f32),
    MoveRightWheel(f32),
    SetLed(bool),
    Stop,
    Rotate(Duration, f32),
    MoveForward(Duration, f32),
    Log(String),
    SaveObstacleData,
    ClearObstacleData,
}

// RuleAction struct
#[derive(Debug, Clone)]
pub struct RuleAction {
    pub action_type: RuleActionType,
    pub priority: u8,
}

// Rule system
#[derive(Debug, Clone)]
pub struct Rule {
    pub name: String,
    pub priority: u8,
    pub condition: Condition,
    pub actions: Vec<RuleAction>,
    pub cooldown: Option<Duration>,
}

// Historical data storage
#[derive(Debug, Clone)]
pub struct HistoricalInput {
    pub input: Input,
    pub timestamp: Instant,
}

#[derive(Debug, Clone)]
pub struct HistoricalRuleAction {
    pub action: RuleAction,
    pub timestamp: Instant,
    pub completed: bool,
}

// Robot state
#[derive(Debug, Clone)]
pub struct RobotState {
    pub position: (f32, f32),
    pub rotation: f32,
    pub obstacle_buffer: Vec<ObstacleData>,
}

#[derive(Debug, Clone)]
pub struct ObstacleData {
    pub timestamp: Instant,
    pub position: (f32, f32),
    pub distance: f32,
}

pub struct EvaluationContext {
    pub current_inputs: Vec<Input>,
    pub historical_inputs: Vec<HistoricalInput>,
    pub historical_actions: Vec<HistoricalRuleAction>,
    pub current_state: RobotState,
}

#[derive(Clone)]
pub enum BrainOutput {
    Live(Output),
    #[cfg(feature = "test")]
    Mock(MockOutput),
}

pub struct Brain {
    current: Vec<CAction>,
    incoming: Vec<CAction>,
    input: Input,
    output: BrainOutput,
    test_mode: bool,
    
    // Add these new fields for the rule system
    pub rules: Vec<Rule>,
    active_rules: HashMap<String, Instant>,
    historical_inputs: VecDeque<HistoricalInput>,
    historical_rule_actions: VecDeque<HistoricalRuleAction>,
    state: RobotState,
}

impl Brain {
    pub async fn init(
        log: Log,
        cfg: BTreeMap<String, String>,
        test_mode: bool,
        time_step: i32,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let current = vec![];
        let incoming = vec![];
        
        // Initialize input based on test mode
        let input = if test_mode {
            #[cfg(feature = "test")]
            {
                let mut input = Input::init_test(time_step)?;
                input.set_test_mode(true);
                input
            }
            #[cfg(not(feature = "test"))]
            Input::init(time_step).await?
        } else {
            Input::init(time_step).await?
        };
        
        #[cfg(feature = "test")]
        let output = if test_mode {
            BrainOutput::Mock(MockOutput::new())
        } else {
            BrainOutput::Live(Output::init(log.clone(), cfg.clone()))
        };
        
        #[cfg(not(feature = "test"))]
        let output = BrainOutput::Live(Output::init(log.clone(), cfg.clone()));
        
        // Initialize new fields for rule system
        let rules = vec![];
        let active_rules = HashMap::new();
        let historical_inputs = VecDeque::new();
        let historical_rule_actions = VecDeque::new();
        let state = RobotState {
            position: (0.0, 0.0),
            rotation: 0.0,
            obstacle_buffer: vec![],
        };
        
        Ok(Self {
            current,
            incoming,
            input,
            output,
            test_mode,
            rules,
            active_rules,
            historical_inputs,
            historical_rule_actions,
            state,
        })
    }

    // Add rule programmatically
    pub fn add_rule(&mut self, rule: Rule) {
        self.rules.push(rule);
        // Sort rules by priority (higher priority first)
        self.rules.sort_by(|a, b| b.priority.cmp(&a.priority));
    }
    
    pub fn add_default_rules(&mut self) {
        // Emergency stop rule
        self.add_rule(Rule {
            name: "emergency_stop".to_string(),
            priority: 100,
            condition: Condition::DistanceLessThan(2000.0),
            actions: vec![
                RuleAction {
                    action_type: RuleActionType::Stop,
                    priority: 100,
                },
                RuleAction {
                    action_type: RuleActionType::Log("Obstacle too close! Emergency stop.".to_string()),
                    priority: 10,
                },
            ],
            cooldown: Some(Duration::from_millis(1000)),
        });
        
        // Normal operation rule
        self.add_rule(Rule {
            name: "normal_operation".to_string(),
            priority: 50,
            condition: Condition::DistanceGreaterThan(2000.0),
            actions: vec![
                RuleAction {
                    action_type: RuleActionType::MoveForward(Duration::from_secs(0), 1.0),
                    priority: 50,
                },
            ],
            cooldown: None,
        });
    }

    async fn evaluate_rules(&mut self) {
        let context = self.create_evaluation_context();
        
        // Clone the rules to avoid borrowing issues
        let rules_clone = self.rules.clone();
        
        for rule in rules_clone {
            // Check cooldown
            if let Some(last_activation) = self.active_rules.get(&rule.name) {
                if let Some(cooldown) = rule.cooldown {
                    if last_activation.elapsed() < cooldown {
                        continue; // Skip rule during cooldown
                    }
                }
            }
            
            // Evaluate condition
            if self.evaluate_condition(&rule.condition, &context) {
                println!("Rule triggered: {}", rule.name);
                
                // Schedule actions
                for action in &rule.actions {
                    self.schedule_rule_action(action.clone());
                }
                
                // Record activation time
                self.active_rules.insert(rule.name.clone(), Instant::now());
                
                // Higher priority rules prevent lower priority ones from firing
                break;
            }
        }
    }
    
    fn create_evaluation_context(&self) -> EvaluationContext {
        EvaluationContext {
            current_inputs: self.get_current_inputs(),
            historical_inputs: self.historical_inputs.iter().cloned().collect(),
            historical_actions: self.historical_rule_actions.iter().cloned().collect(),
            current_state: self.state.clone(),
        }
    }
    
    fn evaluate_condition(&self, condition: &Condition, context: &EvaluationContext) -> bool {
        match condition {
            Condition::DistanceLessThan(value) => {
                self.get_current_distance().map(|d| d < *value).unwrap_or(false)
            }
            Condition::DistanceGreaterThan(value) => {
                self.get_current_distance().map(|d| d > *value).unwrap_or(false)
            }
            Condition::TimeSinceLastAction(duration) => {
                self.get_time_since_last_action().map(|t| t > *duration).unwrap_or(false)
            }
            Condition::ButtonPressed => {
                self.is_button_pressed()
            }
            Condition::EncoderReached(value) => {
                self.get_encoder_value().map(|v| v >= *value).unwrap_or(false)
            }
            Condition::And(a, b) => {
                self.evaluate_condition(a, context) && self.evaluate_condition(b, context)
            }
            Condition::Or(a, b) => {
                self.evaluate_condition(a, context) || self.evaluate_condition(b, context)
            }
            Condition::Not(a) => {
                !self.evaluate_condition(a, context)
            }
            Condition::Always => true,
        }
    }
    
    // Helper methods for condition evaluation
    fn get_current_distance(&self) -> Option<f32> {
        // This should get the actual distance from your input system
        // For now, return a placeholder
        Some(2500.0)
    }
    
    fn get_time_since_last_action(&self) -> Option<Duration> {
        self.historical_rule_actions.back()
            .map(|ha| ha.timestamp.elapsed())
    }
    
    fn is_button_pressed(&self) -> bool {
        false // Placeholder
    }
    
    fn get_encoder_value(&self) -> Option<i32> {
        None // Placeholder
    }
    
    fn get_current_inputs(&self) -> Vec<Input> {
        vec![] // Placeholder
    }
    
    fn schedule_rule_action(&mut self, action: RuleAction) {
        let c_action = self.convert_rule_action_to_caction(action.clone());
        self.record_rule_action(action, false);
        self.add_incoming(c_action);
    }
    
    fn record_rule_action(&mut self, action: RuleAction, completed: bool) {
        let historical_action = HistoricalRuleAction {
            action,
            timestamp: Instant::now(),
            completed,
        };
        
        // Maintain a fixed-size history
        if self.historical_rule_actions.len() >= 1000 {
            self.historical_rule_actions.pop_front();
        }
        self.historical_rule_actions.push_back(historical_action);
    }

    fn convert_rule_action_to_caction(&self, action: RuleAction) -> CAction {
        use RuleActionType::*;
        
        let action_id = format!("rule_action_{}", Instant::now().elapsed().as_nanos());
        
        match action.action_type {
            MoveLeftWheel(speed) => {
                let motor_action = Action::new(
                    format!("{}_motor_l", action_id),
                    0, // delay_ms
                    0, // duration_ms (0 means continuous)
                    "motor_l".to_string(),
                    speed.to_string(),
                );
                
                CAction::new(
                    action_id,
                    vec![motor_action],
                    0, // starts_at (immediately)
                    action.priority,
                )
            }
            MoveRightWheel(speed) => {
                let motor_action = Action::new(
                    format!("{}_motor_r", action_id),
                    0, // delay_ms
                    0, // duration_ms (0 means continuous)
                    "motor_r".to_string(),
                    speed.to_string(),
                );
                
                CAction::new(
                    action_id,
                    vec![motor_action],
                    0, // starts_at (immediately)
                    action.priority,
                )
            }
            SetLed(state) => {
                let led_action = Action::new(
                    format!("{}_led", action_id),
                    0, // delay_ms
                    0, // duration_ms
                    "led".to_string(),
                    if state { "1" } else { "0" }.to_string(),
                );
                
                CAction::new(
                    action_id,
                    vec![led_action],
                    0, // starts_at (immediately)
                    action.priority,
                )
            }
            Stop => {
                let stop_left = Action::new(
                    format!("{}_stop_l", action_id),
                    0, // delay_ms
                    0, // duration_ms
                    "motor_l".to_string(),
                    "0".to_string(),
                );
                
                let stop_right = Action::new(
                    format!("{}_stop_r", action_id),
                    0, // delay_ms
                    0, // duration_ms
                    "motor_r".to_string(),
                    "0".to_string(),
                );
                
                CAction::new(
                    action_id,
                    vec![stop_left, stop_right],
                    0, // starts_at (immediately)
                    action.priority,
                )
            }
            Rotate(duration, speed) => {
                let rotate_left = Action::new(
                    format!("{}_rotate_l", action_id),
                    0, // delay_ms
                    duration.as_millis() as u128, // duration_ms
                    "motor_l".to_string(),
                    speed.to_string(),
                );
                
                let rotate_right = Action::new(
                    format!("{}_rotate_r", action_id),
                    0, // delay_ms
                    duration.as_millis() as u128, // duration_ms
                    "motor_r".to_string(),
                    (-speed).to_string(), // opposite direction
                );
                
                CAction::new(
                    action_id,
                    vec![rotate_left, rotate_right],
                    0, // starts_at (immediately)
                    action.priority,
                )
            }
            MoveForward(duration, speed) => {
                let move_left = Action::new(
                    format!("{}_forward_l", action_id),
                    0, // delay_ms
                    duration.as_millis() as u128, // duration_ms
                    "motor_l".to_string(),
                    speed.to_string(),
                );
                
                let move_right = Action::new(
                    format!("{}_forward_r", action_id),
                    0, // delay_ms
                    duration.as_millis() as u128, // duration_ms
                    "motor_r".to_string(),
                    speed.to_string(),
                );
                
                CAction::new(
                    action_id,
                    vec![move_left, move_right],
                    0, // starts_at (immediately)
                    action.priority,
                )
            }
            Log(message) => {
                let log_action = Action::new(
                    format!("{}_log", action_id),
                    0, // delay_ms
                    0, // duration_ms
                    "log".to_string(),
                    message,
                );
                
                CAction::new(
                    action_id,
                    vec![log_action],
                    0, // starts_at (immediately)
                    action.priority,
                )
            }
            SaveObstacleData => {
                let save_action = Action::new(
                    format!("{}_save_obstacle", action_id),
                    0, // delay_ms
                    0, // duration_ms
                    "save_obstacle".to_string(),
                    "".to_string(),
                );
                
                CAction::new(
                    action_id,
                    vec![save_action],
                    0, // starts_at (immediately)
                    action.priority,
                )
            }
            ClearObstacleData => {
                let clear_action = Action::new(
                    format!("{}_clear_obstacle", action_id),
                    0, // delay_ms
                    0, // duration_ms
                    "clear_obstacle".to_string(),
                    "".to_string(),
                );
                
                CAction::new(
                    action_id,
                    vec![clear_action],
                    0, // starts_at (immediately)
                    action.priority,
                )
            }
        }
    }

    pub async fn update(&mut self, log: Log, add_incoming: String) -> BrainOutput {
        // We avoid doing this while testing, for higher control on tests
        // TODO: remove test_mode, possible?
        if !self.test_mode && add_incoming == "on" {
            //for ac in self.input.react(log.clone()) {
            //    self.add_incoming(ac);
            //}
        }
        let (_ts, dist) = self.input.update().await;
        log.info(&format!(" Distance: {:?}", dist));
        
        // Evaluate rules instead of hardcoded logic
        self.evaluate_rules().await;
        
        // TODO: check more than one distance
        if dist[0] < 2000.0 {
            match &mut self.output {
                BrainOutput::Live(out) => {
                    out.set_motor_l(1.00, 1);
                    out.set_motor_r(-1.00, 1);
                }
                #[cfg(feature = "test")]
                BrainOutput::Mock(out) => {
                    out.set_motor("motor_l", 1.00);
                    out.set_motor("motor_r", -1.00);
                }
            }
        } else {
            match &mut self.output {
                BrainOutput::Live(out) => {
                    out.set_motor_l(1.00, 1);
                    out.set_motor_r(1.00, 1);
                }
                #[cfg(feature = "test")]
                BrainOutput::Mock(out) => {
                    out.set_motor("motor_l", 1.00);
                    out.set_motor("motor_r", 1.00);
                }
            }
        };
        
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

    pub fn get_output(&self) -> BrainOutput {
        return self.output.clone();
    }

    #[cfg(feature = "test")]
    pub fn get_mock_output(&self) -> Option<&MockOutput> {
        match &self.output {
            BrainOutput::Mock(out) => Some(out),
            _ => None,
        }
    }

        // Add a method to set mock input data for testing
    #[cfg(feature = "test")]
    pub fn set_mock_input_data(&mut self, timestamp: Duration, distances: Vec<f64>) {
        if self.test_mode {
            self.input.set(timestamp, distances);
        }
    }
    
    // Add a method to get the current input state for verification
    #[cfg(feature = "test")]
    pub fn get_input_state(&mut self) -> (Duration, Vec<f64>) {
        (self.input.get_ts(), self.input.get_distance())
    }
}
