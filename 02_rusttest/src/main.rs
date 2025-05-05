use gpio_cdev::{Chip, LineRequestFlags};
use std::{thread::sleep, time::Duration};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open the GPIO chip (e.g., /dev/gpiochip0)
    let mut chip = Chip::new("/dev/gpiochip0")?;

    // Define the GPIO line offset (GPIO4 corresponds to offset 4)
    let led_pin = 4;

    // Request the line as output, initial value 0
    let handle = chip
        .get_line(led_pin)?
        .request(LineRequestFlags::OUTPUT, 0, "LED")?;

    // Blink loop
    loop {
        handle.set_value(1)?; // Turn LED on
        sleep(Duration::from_secs(1));

        handle.set_value(0)?; // Turn LED off
        sleep(Duration::from_secs(1));
    }
}
