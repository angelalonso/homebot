use super::{InputSource, SensorData};

/// Webots simulation input backend
pub struct WebotsInput {
    // Webots robot instance would be stored here
    // For now, we'll simulate encoder data
    left_encoder: i32,
    right_encoder: i32,
    simulation_time: u64,
}

impl WebotsInput {
    pub fn new() -> Result<Self, anyhow::Error> {
        // TODO: Initialize Webots robot instance
        Ok(Self {
            left_encoder: 0,
            right_encoder: 0,
            simulation_time: 0,
        })
    }
    
    /// Simulate encoder reading based on motor commands
    pub fn update_simulation(&mut self, left_speed: i8, right_speed: i8) {
        // Simple simulation: speed directly affects encoder ticks
        self.left_encoder += left_speed as i32;
        self.right_encoder += right_speed as i32;
        self.simulation_time += 10; // 10ms increments
    }
}

impl InputSource for WebotsInput {
    fn read_encoders(&mut self) -> Result<(i32, i32), anyhow::Error> {
        Ok((self.left_encoder, self.right_encoder))
    }
    
    fn read_sensors(&mut self) -> Result<SensorData, anyhow::Error> {
        Ok(SensorData {
            left_encoder: self.left_encoder,
            right_encoder: self.right_encoder,
            timestamp_ms: self.simulation_time,
        })
    }
    
    fn is_connected(&self) -> bool {
        true // Webots is always "connected" in simulation
    }
}
