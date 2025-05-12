use gpio_cdev::{Chip, LineRequestFlags};
use tokio_serial::{SerialPortBuilderExt, SerialStream};
use tokio::io::{AsyncBufReadExt, BufReader};

const LED_GPIO: u32 = 4;  // GPIO4 (Pin 7 on Raspberry Pi)

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // GPIO Setup
    let mut chip = Chip::new("/dev/gpiochip0")?;
    let line = chip.get_line(LED_GPIO)?;
    let mut led = line.request(LineRequestFlags::OUTPUT, 0, "distance_led")?;

    // Serial Setup - replace "/dev/ttyACM0" with your actual port
    let mut port = tokio_serial::new("/dev/ttyACM0", 9600)
        .open_native_async()?;

    let mut reader = BufReader::new(port);
    let mut line = String::new();

    loop {
        line.clear();
        reader.read_line(&mut line).await?;
        
        if line.starts_with("Distance: ") {
            if let Some(distance_str) = line.split_whitespace().nth(1) {
                if let Ok(distance) = distance_str.parse::<f32>() {
                    println!("Distance: {} cm", distance);
                    
                    // Control LED
                    led.set_value(if distance < 20.0 { 1 } else { 0 })?;
                }
            }
        }
    }
}
