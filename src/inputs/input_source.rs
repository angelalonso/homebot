/// Trait for all input sources (Webots simulation, Arduino serial, etc.)
pub trait InputSource: Send {
    /// Read encoder values from the input source
    /// Returns (left_encoder_ticks, right_encoder_ticks)
    fn read_encoders(&mut self) -> Result<(i32, i32), anyhow::Error>;
    
    /// Read any additional sensor data
    fn read_sensors(&mut self) -> Result<SensorData, anyhow::Error>;
    
    /// Check if input source is connected and ready
    fn is_connected(&self) -> bool;
}

#[derive(Debug, Clone, Default)]
pub struct SensorData {
    pub left_encoder: i32,
    pub right_encoder: i32,
    pub timestamp_ms: u64,
    // Add other sensors as needed: IMU, distance sensors, etc.
}
