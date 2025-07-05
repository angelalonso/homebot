//use gpio_cdev::{Chip, LineRequestFlags};
//
use gpio_cdev::Chip;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::time::{timeout, Duration};
use tokio_serial::{SerialPortBuilderExt, SerialStream};

use crate::bindings;
use crate::error::AppError;

// - Robot general functions
pub async fn get_serial_port(_time_step: i32) -> Result<(String, Vec<u16>), AppError> {
    let ports = tokio_serial::available_ports()?;
    ports
        .into_iter()
        .find(|p| p.port_name.contains("ACM") || p.port_name.contains("USB"))
        .map(|p| (p.port_name, Vec::<u16>::default()))
        .ok_or_else(|| AppError::Config("No Arduino found".into()))
}

pub async fn read_distance(serial_port: &str, _sensor_ids: Vec<u16>, _time_step: i32) -> Vec<f64> {
    let mut result: Vec<f64> = Vec::new();
    let r = distance_sensor_get_value(serial_port).await;
    let rd = r
        .expect("ERROR Reading from Serial")
        .get("Di") // Distance
        .copied()
        .unwrap_or(0.0);
    result.push(rd);
    return result;
}

pub fn robot_init() {
    ();
}

pub fn robot_step(_step: i32) -> i32 {
    0
}

pub fn robot_cleanup() {
    ();
}

pub async fn read_distance_binary(
    port: &mut SerialStream,
) -> Result<f64, Box<dyn std::error::Error>> {
    let mut header = [0u8; 1];
    port.read_exact(&mut header).await?;

    if header[0] != b'D' {
        return Err("Invalid header".into());
    }

    let mut bytes = [0u8; 4];
    port.read_exact(&mut bytes).await?;

    Ok(f32::from_le_bytes(bytes) as f64)
    // or from_be_bytes() depending on Arduino's endianness
}

// - Sensors functions
lazy_static! {
    // Strict regex that requires proper key-value pairs
    static ref KV_REGEX: Regex = Regex::new(r"^([A-Za-z]{2}):([^|\r\n]+)$").unwrap();
}

pub async fn distance_sensor_get_value(
    serial_port: &str,
) -> Result<HashMap<String, f64>, Box<dyn std::error::Error>> {
    let mut port = tokio_serial::new(serial_port, 115_200).open_native_async()?;
    port.set_exclusive(false)?;

    let mut reader = BufReader::new(port);
    let mut line = String::new();

    reader.read_line(&mut line).await?;

    let mut data = HashMap::new();

    for part in line.split('|') {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }
        match KV_REGEX.captures(part) {
            Some(caps) => {
                let key = caps.get(1).unwrap().as_str();
                let value_str = caps.get(2).unwrap().as_str().trim();

                match (key, value_str) {
                    ("Di", v) => {
                        if let Ok(val) = v.parse::<f64>() {
                            data.insert(key.to_string(), val);
                        } else {
                            println!("Discarding invalid Di value: {}", v);
                        }
                    }
                    ("St", "OK") => {
                        data.insert(key.to_string(), 1.0);
                    }
                    ("St", _) => {
                        data.insert(key.to_string(), 0.0);
                    }
                    _ => {
                        println!("Discarding unknown key: {}", key);
                    }
                }
            }
            None => {
                if !part.is_empty() {
                    println!("Discarding malformed entry: {:?}", part);
                }
            }
        }
    }

    // Validate required fields
    if !data.contains_key("Di") || !data.contains_key("St") {
        return Err(format!(
            "Missing required fields (have Di: {}, St: {})",
            data.contains_key("Di"),
            data.contains_key("St")
        )
        .into());
    }

    Ok(data)
}

// - Motor functions
pub fn motor_set_velocity(
    pins: (u32, u32, u32),
    velocity: f64,
) -> Result<(), Box<dyn std::error::Error>> {
    // TODO: get this from outside
    let mut chip: Chip = gpio_cdev::Chip::new("/dev/gpiochip0")?;
    //     // bindings_live, also check 04 for how we do that
    let mut motor = bindings::Motor::new(&mut chip, pins.0, pins.1, pins.2)?;
    let _ = motor.set_speed(velocity as i8);
    Ok(())
}

// - Arduino functions
pub struct Arduino {
    port: SerialStream,
}

impl Arduino {
    pub async fn new(port_path: &str) -> Result<Self, AppError> {
        let port = tokio_serial::new(port_path, 115200).open_native_async()?;
        Ok(Self { port })
    }

    pub async fn read_distance(&mut self) -> Result<Option<f32>, AppError> {
        let mut buf = [0u8; 5];
        match timeout(Duration::from_millis(200), self.port.read_exact(&mut buf)).await {
            Ok(Ok(_)) if buf[0] == b'D' => {
                Ok(Some(f32::from_le_bytes([buf[1], buf[2], buf[3], buf[4]])))
            }
            Ok(Ok(_)) => Ok(None),       // Invalid header
            Ok(Err(e)) => Err(e.into()), // This is what was missing
            Err(e) => Err(e.into()),     // Timeout
        }
    }

    pub async fn send_ping(&mut self) -> Result<(), AppError> {
        self.port.write_all(&[b'P']).await?;
        Ok(())
    }
}
