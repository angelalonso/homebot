use super::{InputSource, SensorData};
use serialport::{SerialPort, SerialPortType};
use std::io::{BufRead, BufReader};
use std::time::Duration;

/// Arduino serial input backend
pub struct ArduinoInput {
    port: Box<dyn SerialPort>,
    reader: BufReader<Box<dyn SerialPort>>,
}

impl ArduinoInput {
    pub fn new(port_name: &str, baud_rate: u32) -> Result<Self, anyhow::Error> {
        let port = serialport::new(port_name, baud_rate)
            .timeout(Duration::from_millis(10))
            .open()?;
            
        let reader = BufReader::new(port.try_clone()?);
        
        Ok(Self {
            port,
            reader,
        })
    }
    
    /// Parse encoder data from Arduino serial protocol
    /// Expected format: "ENC <left> <right>"
    fn parse_encoder_data(&mut self) -> Result<Option<(i32, i32)>, anyhow::Error> {
        let mut line = String::new();
        
        if self.reader.read_line(&mut line)? > 0 {
            let parts: Vec<&str> = line.trim().split_whitespace().collect();
            
            if parts.len() == 3 && parts[0] == "ENC" {
                let left = parts[1].parse::<i32>()?;
                let right = parts[2].parse::<i32>()?;
                return Ok(Some((left, right)));
            }
        }
        
        Ok(None)
    }
}

impl InputSource for ArduinoInput {
    fn read_encoders(&mut self) -> Result<(i32, i32), anyhow::Error> {
        match self.parse_encoder_data()? {
            Some((left, right)) => Ok((left, right)),
            None => Ok((0, 0)), // No new data
        }
    }
    
    fn read_sensors(&mut self) -> Result<SensorData, anyhow::Error> {
        let (left, right) = self.read_encoders()?;
        
        Ok(SensorData {
            left_encoder: left,
            right_encoder: right,
            timestamp_ms: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)?
                .as_millis() as u64,
        })
    }
    
    fn is_connected(&self) -> bool {
        // Check if serial port is still available
        !matches!(self.port.port_type(), SerialPortType::Unknown)
    }
}
