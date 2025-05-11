use gpio_cdev::{Chip, LineRequestFlags};
use regex::Regex;
use serialport::{SerialPort, SerialPortInfo, SerialPortType};
use std::error::Error;
use std::time::Duration;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

const LED_GPIO: u32 = 4; // GPIO4 (Pin 7 on Raspberry Pi)

/// Auto-detect Arduino's serial port
fn find_arduino_port() -> Option<SerialPortInfo> {
    let ports = serialport::available_ports().unwrap();
    let arduino_regex = Regex::new(r"(ACM|USB|arduino)").unwrap();

    ports.into_iter().find(|port| match &port.port_type {
        SerialPortType::UsbPort(info) => {
            arduino_regex.is_match(&info.product.unwrap_or_default().to_lowercase())
        }
        _ => false,
    })
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // GPIO Setup (LED) using gpio-cdev
    let mut chip = Chip::new("/dev/gpiochip0")?;
    let line = chip.get_line(LED_GPIO)?;
    let mut led = line.request(LineRequestFlags::OUTPUT, 0, "distance_led")?;

    // Auto-detect Arduino
    let arduino_port = find_arduino_port().ok_or("Arduino not found!")?;
    println!("Found Arduino at: {}", arduino_port.port_name);

    // Open serial connection
    let mut port = serialport::new(arduino_port.port_name, 9600)
        .timeout(Duration::from_millis(1000))
        .open()?;

    // Async reader for serial data
    let mut reader = BufReader::new(port.try_clone()?);
    let mut line = String::new();

    loop {
        line.clear();
        if reader.read_line(&mut line).await? > 0 {
            if line.starts_with("Distance: ") {
                if let Some(distance_str) = line.split_whitespace().nth(1) {
                    if let Ok(distance) = distance_str.parse::<f32>() {
                        println!("Distance: {} cm", distance);

                        // Turn LED on if distance < 20 cm
                        led.set_value(if distance < 20.0 { 1 } else { 0 })?;
                    }
                }
            }
        }
    }
}
