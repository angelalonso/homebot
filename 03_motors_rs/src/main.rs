use gpio_cdev::{Chip, LineRequestFlags};
use tokio_serial::{SerialPortBuilderExt, SerialStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::time::{timeout, Duration};
use std::error::Error;

// GPIO Pins (BCM numbering)
const LED_PIN: u32 = 4;
const MOTOR_A_ENABLE: u32 = 17;
const MOTOR_A_IN1: u32 = 27;
const MOTOR_A_IN2: u32 = 22;
const MOTOR_B_ENABLE: u32 = 23;
const MOTOR_B_IN1: u32 = 24;
const MOTOR_B_IN2: u32 = 25;

const STOP_DISTANCE_CM: f32 = 20.0;
const SERIAL_TIMEOUT: Duration = Duration::from_millis(100);
const SENSOR_TIMEOUT: Duration = Duration::from_millis(200);

struct MotorController {
    enable: gpio_cdev::LineHandle,
    in1: gpio_cdev::LineHandle,
    in2: gpio_cdev::LineHandle,
}

impl MotorController {
    fn new(chip: &mut Chip, enable_pin: u32, in1_pin: u32, in2_pin: u32) -> Result<Self, gpio_cdev::Error> {
        Ok(Self {
            enable: chip.get_line(enable_pin)?.request(LineRequestFlags::OUTPUT, 0, "motor_enable")?,
            in1: chip.get_line(in1_pin)?.request(LineRequestFlags::OUTPUT, 0, "motor_in1")?,
            in2: chip.get_line(in2_pin)?.request(LineRequestFlags::OUTPUT, 0, "motor_in2")?,
        })
    }

    fn set_speed(&mut self, speed: i8) -> Result<(), gpio_cdev::Error> {
        match speed {
            s if s > 0 => { // Forward
                self.in1.set_value(1)?;
                self.in2.set_value(0)?;
                self.enable.set_value(1)?;
            },
            s if s < 0 => { // Reverse
                self.in1.set_value(0)?;
                self.in2.set_value(1)?;
                self.enable.set_value(1)?;
            },
            _ => { // Stop
                self.enable.set_value(0)?;
            }
        }
        Ok(())
    }
}

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
    
    // LED
    let led = chip.get_line(LED_PIN)?.request(LineRequestFlags::OUTPUT, 0, "led")?;
    
    // Motors
    let mut motor_a = MotorController::new(&mut chip, MOTOR_A_ENABLE, MOTOR_A_IN1, MOTOR_A_IN2)?;
    let mut motor_b = MotorController::new(&mut chip, MOTOR_B_ENABLE, MOTOR_B_IN1, MOTOR_B_IN2)?;

    // Serial Setup
    let port_path = find_arduino_port().await?;
    let mut port = tokio_serial::new(&port_path, 115200).open_native_async()?;

    let mut buf = [0u8; 5]; // 'D' + 4-byte float
    let mut error_count = 0;

    // Start motors forward
    motor_a.set_speed(50)?;
    motor_b.set_speed(50)?;

    loop {
        match timeout(SENSOR_TIMEOUT, port.read_exact(&mut buf)).await {
            Ok(Ok(_)) if buf[0] == b'D' => {
                error_count = 0;
                let distance = f32::from_le_bytes([buf[1], buf[2], buf[3], buf[4]]);
                
                // Control LED and motors based on distance
                let should_stop = distance < STOP_DISTANCE_CM;
                led.set_value(if should_stop { 1 } else { 0 })?;
                
                if should_stop {
                    motor_a.set_speed(0)?;
                    motor_b.set_speed(0)?;
                } else {
                    motor_a.set_speed(50)?;
                    motor_b.set_speed(50)?;
                }

                println!("Distance: {:.1}cm - Motors: {}", distance, if should_stop { "STOPPED" } else { "RUNNING" });
            }
            Ok(Ok(_)) => {
                error_count += 1;
                if error_count > 5 {
                    eprintln!("Protocol errors, stopping motors");
                    motor_a.set_speed(0)?;
                    motor_b.set_speed(0)?;
                    port.write_all(&[b'R']).await?;
                    error_count = 0;
                }
            }
            Err(_) => {
                eprintln!("Timeout, stopping motors for safety");
                motor_a.set_speed(0)?;
                motor_b.set_speed(0)?;
                port.write_all(&[b'P']).await?;
                match timeout(SERIAL_TIMEOUT, port.read_u8()).await {
                    Ok(Ok(b'A')) => continue,
                    _ => return Err("Sensor disconnected".into()),
                }
            }
            Ok(Err(e)) => return Err(e.into()),

        }
    }
}
