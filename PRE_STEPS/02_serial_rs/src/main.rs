use gpio_cdev::{Chip, LineRequestFlags};
use tokio_serial::{SerialPortBuilderExt, SerialStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::time::{timeout, Duration};
use std::error::Error;

const LED_GPIO: u32 = 4;
const SERIAL_TIMEOUT: Duration = Duration::from_millis(100);
const SENSOR_TIMEOUT: Duration = Duration::from_millis(200);

async fn find_arduino_port() -> Result<String, Box<dyn Error>> {
    let ports = tokio_serial::available_ports()?;
    ports.into_iter()
        .find(|p| p.port_name.contains("ACM") || p.port_name.contains("USB"))
        .map(|p| p.port_name)
        .ok_or("No Arduino found".into())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // GPIO Setup
    let mut chip = Chip::new("/dev/gpiochip0")?;
    let line = chip.get_line(LED_GPIO)?;
    let mut led = line.request(LineRequestFlags::OUTPUT, 0, "distance_led")?;

    // Serial Setup
    let port_path = find_arduino_port().await?;
    let mut port = tokio_serial::new(&port_path, 115200)
        .open_native_async()?;

    let mut buf = [0u8; 5]; // 'D' + 4-byte float
    let mut error_count = 0;

    loop {
        match timeout(SENSOR_TIMEOUT, port.read_exact(&mut buf)).await {
            Ok(Ok(_)) if buf[0] == b'D' => {
                error_count = 0;
                let distance = f32::from_le_bytes([buf[1], buf[2], buf[3], buf[4]]);
                led.set_value(if distance < 20.0 { 1 } else { 0 })?;
                
                // Add future processing here
                println!("Distance: {:.1}cm", distance);
            }
            Ok(Ok(_)) => {  // Invalid header
                error_count += 1;
                if error_count > 5 {
                    eprintln!("Too many protocol errors, resetting...");
                    port.write_all(&[b'R']).await?; // Optional reset signal
                    error_count = 0;
                }
            }
            Err(_) => {  // Timeout
                eprintln!("Sensor timeout, checking connection...");
                port.write_all(&[b'P']).await?; // Ping
                match timeout(SERIAL_TIMEOUT, port.read_u8()).await {
                    Ok(Ok(b'A')) => continue,  // Got ack
                    _ => return Err("Sensor disconnected".into()),
                }
            }
            Err(e) => return Err(e.into()),
        }
    }
}
