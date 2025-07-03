//use gpio_cdev::{Chip, LineRequestFlags};
use gpio_cdev::Chip;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::time::{timeout, Duration};
use tokio_serial::{SerialPortBuilderExt, SerialStream};

use crate::bindings;
use crate::error::AppError;

// - Robot general functions
pub async fn find_port(_time_step: i32) -> Result<String, AppError> {
    let ports = tokio_serial::available_ports()?;
    ports
        .into_iter()
        .find(|p| p.port_name.contains("ACM") || p.port_name.contains("USB"))
        .map(|p| p.port_name)
        .ok_or_else(|| AppError::Config("No Arduino found".into()))
}

pub async fn find_distance_sensor(time_step: i32, name: &str) -> Result<String, AppError> {
    Ok(name.to_string()) // TODO: make back and forth u16 to string
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

// - Sensors functions
pub fn distance_sensor_get_value(_tag: &str) -> f64 {
    // TODO: get it from Arduino
    0.0
}

pub fn read_distance(sensor: &str, time_step: i32) -> f64 {
    let distance_values = distance_sensor_get_value(sensor);
    return distance_values;
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
