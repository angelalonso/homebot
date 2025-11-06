use crate::config::Config;

#[derive(Debug, Clone)]
pub struct MotorCommand {
    pub left_speed: i16,
    pub right_speed: i16,
}

pub struct MotorControl {
    config: Config,
    left_encoder: i32,
    right_encoder: i32,
    last_left_encoder: i32,
    last_right_encoder: i32,
}

impl MotorControl {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            left_encoder: 0,
            right_encoder: 0,
            last_left_encoder: 0,
            last_right_encoder: 0,
        }
    }
    
    pub fn update_encoders(&mut self, left: i32, right: i32) {
        self.last_left_encoder = self.left_encoder;
        self.last_right_encoder = self.right_encoder;
        self.left_encoder = left;
        self.right_encoder = right;
    }
    
    /// Compute motor commands based on desired behavior
    /// This is where you'll implement your robot logic
    pub fn compute_motor_commands(&self, desired_speed: f32, desired_turn: f32) -> MotorCommand {
        // Simple differential drive control
        let base_speed = (desired_speed * self.config.actions.max_motor_speed as f32) as i16;
        let turn_adjustment = (desired_turn * self.config.actions.max_motor_speed as f32) as i16;
        
        let left_speed = (base_speed - turn_adjustment).clamp(-255, 255);
        let right_speed = (base_speed + turn_adjustment).clamp(-255, 255);
        
        MotorCommand {
            left_speed,
            right_speed,
        }
    }
    
    /// Get encoder deltas since last update
    pub fn get_encoder_deltas(&self) -> (i32, i32) {
        (
            self.left_encoder - self.last_left_encoder,
            self.right_encoder - self.last_right_encoder,
        )
    }
    
    /// Convert encoder ticks to distance in mm
    pub fn encoder_to_distance(&self, ticks: i32) -> f32 {
        (ticks as f32 / self.config.actions.encoder_ticks_per_revolution as f32)
            * self.config.actions.wheel_circumference_mm
    }
}
