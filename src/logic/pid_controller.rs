#[derive(Debug, Clone)]
pub struct PidController {
    kp: f32,
    ki: f32,
    kd: f32,
    integral: f32,
    last_error: f32,
    output_limit: Option<f32>,
}

impl PidController {
    pub fn new(kp: f32, ki: f32, kd: f32) -> Self {
        Self {
            kp,
            ki,
            kd,
            integral: 0.0,
            last_error: 0.0,
            output_limit: None,
        }
    }
    
    pub fn with_limits(mut self, min: f32, max: f32) -> Self {
        self.output_limit = Some(max.abs().max(min.abs()));
        self
    }
    
    pub fn compute(&mut self, error: f32, dt: f32) -> f32 {
        self.integral += error * dt;
        let derivative = (error - self.last_error) / dt;
        self.last_error = error;
        
        let output = (self.kp * error) + (self.ki * self.integral) + (self.kd * derivative);
        
        if let Some(limit) = self.output_limit {
            output.clamp(-limit, limit)
        } else {
            output
        }
    }
    
    pub fn reset(&mut self) {
        self.integral = 0.0;
        self.last_error = 0.0;
    }
}
