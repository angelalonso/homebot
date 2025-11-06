/// Trait for all action targets (Webots simulation, Arduino motor control, etc.)
pub trait ActionTarget: Send {
    /// Set motor speeds (-255 to 255)
    fn set_motor_speeds(&mut self, left_speed: i16, right_speed: i16) -> Result<(), anyhow::Error>;
    
    /// Stop all motors
    fn stop_motors(&mut self) -> Result<(), anyhow::Error> {
        self.set_motor_speeds(0, 0)
    }
    
    /// Get current motor speeds
    fn get_motor_speeds(&self) -> (i16, i16);
    
    /// Reset encoders (if supported)
    fn reset_encoders(&mut self) -> Result<(), anyhow::Error>;
}
