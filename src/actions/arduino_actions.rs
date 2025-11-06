use super::ActionTarget;
use serialport::{SerialPort, SerialPortType};
use std::time::Duration;

/// Arduino serial action backend
pub struct ArduinoActions {
    port: Box<dyn SerialPort>,
}

impl ArduinoActions {
    pub fn new(port_name: &str, baud_rate: u32) -> Result<Self, anyhow::Error> {
        let port = serialport::new(port_name, baud_rate)
            .timeout(Duration::from_millis(100))
            .open()?;
            
        Ok(Self { port })
    }
    
    /// Send motor command to Arduino
    /// Protocol: "MOT <left> <right>"
    fn send_motor_command(&mut self, left_speed: i16, right_speed: i16) -> Result<(), anyhow::Error> {
        let command = format!("MOT {} {}\n", left_speed, right_speed);
        self.port.write_all(command.as_bytes())?;
        Ok(())
    }
}

impl ActionTarget for ArduinoActions {
    fn set_motor_speeds(&mut self, left_speed: i16, right_speed: i16) -> Result<(), anyhow::Error> {
        self.send_motor_command(left_speed, right_speed)
    }
    
    fn get_motor_speeds(&self) -> (i16, i16) {
        // Arduino doesn't report back current speeds in this simple protocol
        // You might want to extend the protocol to include feedback
        (0, 0)
    }
    
    fn reset_encoders(&mut self) -> Result<(), anyhow::Error> {
        let command = "RST\n";
        self.port.write_all(command.as_bytes())?;
        Ok(())
    }
}
