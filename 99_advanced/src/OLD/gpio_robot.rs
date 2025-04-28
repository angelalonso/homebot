use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[cfg(target_arch = "x86_64")]
use crate::mock_rust_pigpio::initialize;
#[cfg(target_arch = "x86_64")]
use crate::mock_rust_pigpio::pwm;

#[cfg(target_arch = "arm")]
use rust_pigpio::initialize;
#[cfg(target_arch = "arm")]
use rust_pigpio::pwm;

#[cfg(target_arch = "aarch64")]
use rust_pigpio::initialize;
#[cfg(target_arch = "aarch64")]
use rust_pigpio::pwm;

// Thanks to: https://michm.de/blog/rust/ansteuern-von-raspberry-pi-gpio-in-rust/

pub type Directions = self::directions::Directions;

pub mod directions {
    pub enum Directions {
        Input,
        Output,
    }

    impl Directions {
        pub fn as_str(&self) -> &'static str {
            match *self {
                Directions::Input => "in",
                Directions::Output => "out",
            }
        }

        pub fn as_bytes(&self) -> &[u8] {
            self.as_str().as_bytes()
        }
    }
}

pub struct GPIOLed {
    pin: u8,
    is_real: bool,
}

impl GPIOLed {
    pub fn new(pin: u8) -> Self {
        let is_real = export(pin);
        if is_real {
            std::thread::sleep(std::time::Duration::from_millis(50));
            set_direction(pin, Directions::Output);
        }
        GPIOLed { pin, is_real }
    }

    pub fn on(&self) {
        if self.is_real {
            write(self.pin, 1);
        } else {
            println!(" Mocked ON, pin {}", self.pin);
        }
    }

    pub fn off(&self) {
        if self.is_real {
            write(self.pin, 0);
        } else {
            println!(" Mocked OFF, pin {}", self.pin);
        }
    }
}

pub struct GPIOMotor {
    pin1: u8,
    pin2: u8,
    pin_enabler: u8,
    is_real: bool,
}

impl GPIOMotor {
    pub fn new(pin1: u8, pin2: u8, pin_enabler: u8) -> Self {
        let is_real: bool;
        std::thread::sleep(std::time::Duration::from_millis(50));
        let out1 = export(pin1);
        std::thread::sleep(std::time::Duration::from_millis(50));
        let out2 = export(pin2);
        std::thread::sleep(std::time::Duration::from_millis(50));
        let out_enabler = export(pin_enabler);
        std::thread::sleep(std::time::Duration::from_millis(50));
        if out1 && out2 && out_enabler {
            initialize();
            is_real = true;
            std::thread::sleep(std::time::Duration::from_millis(50));
            set_direction(pin1, Directions::Output);
            set_direction(pin2, Directions::Output);
            set_direction(pin_enabler, Directions::Output);
            std::thread::sleep(std::time::Duration::from_millis(50));
            write(pin1, 0);
            write(pin2, 0);
            pwm::set_pwm_frequency(pin_enabler as u32, 1000).unwrap();
            pwm::set_pwm_range(pin_enabler as u32, 2000).unwrap(); //     Set range to 1000. 1 range = 2 us;
            let _p = pwm::pwm(pin_enabler as u32, 2000);
        } else {
            is_real = false;
        }
        GPIOMotor {
            pin1,
            pin2,
            pin_enabler,
            is_real,
        }
    }

    pub fn fwd(&self) {
        if self.is_real {
            write(self.pin1, 0);
            write(self.pin2, 1);
        } else {
            println!(
                " Mocked FWD, pins {},{} - {}",
                self.pin1, self.pin2, self.pin_enabler
            );
        }
    }

    pub fn bwd(&self) {
        if self.is_real {
            write(self.pin1, 1);
            write(self.pin2, 0);
        } else {
            println!(
                " Mocked BWD, pins {},{} - {}",
                self.pin1, self.pin2, self.pin_enabler
            );
        }
    }

    pub fn stp(&self) {
        if self.is_real {
            write(self.pin1, 0);
            write(self.pin2, 0);
        } else {
            println!(
                " Mocked STP, pins {},{} - {}",
                self.pin1, self.pin2, self.pin_enabler
            );
        }
    }
}
// AUX Functions

fn open_file(filepath: &String) -> Result<File, std::io::Error> {
    let path = Path::new(&filepath);
    let file: File = match fs::OpenOptions::new().write(true).open(path) {
        Err(why) => return Err(why),
        Ok(f) => f,
    };

    Ok(file)
}

pub fn export(gpio_num: u8) -> bool {
    let mut is_real: bool = false;
    let mut file: File;
    let filepath = String::from("/sys/class/gpio/export");

    match open_file(&filepath) {
        Err(why) => println!("couldn't open {}: {}", filepath, why),
        Ok(f) => {
            file = f;
            if let Err(why) = file.write_all(gpio_num.to_string().as_bytes()) {
                println!("couldn't write to {}: {} ({})", filepath, why, gpio_num);
                is_real = false;
            } else {
                is_real = true;
            }
        }
    }
    is_real
}

pub fn unexport(gpio_num: u8) {
    let filepath = String::from("/sys/class/gpio/unexport");

    let mut file: File = match open_file(&filepath) {
        Err(why) => panic!("couldn't open {}: {}", filepath, why),
        Ok(f) => f,
    };

    if let Err(why) = file.write_all(gpio_num.to_string().as_bytes()) {
        panic!("couldn't write to {}: {}", filepath, why);
    }
}

pub fn write(gpio_num: u8, signal: u8) {
    let mut filepath = String::from("/sys/class/gpio/gpio%/value");
    let gpio = gpio_num.to_string();
    filepath = filepath.replace('%', gpio.as_str());

    let mut file: File = match open_file(&filepath) {
        Err(why) => panic!("couldn't open {}: {}", filepath, why),
        Ok(f) => f,
    };

    if let Err(why) = file.write_all(signal.to_string().as_bytes()) {
        panic!("couldn't write to {}: {}", filepath, why);
    }
}

pub fn read(gpio_num: u8) -> bool {
    let mut filepath = String::from("/sys/class/gpio/gpio%/value");
    let gpio = gpio_num.to_string();
    filepath = filepath.replace('%', gpio.as_str());

    match fs::read_to_string(filepath) {
        Err(why) => panic!("couldn't read: {}", why),
        Ok(contents) => contents.parse::<u8>().unwrap() > 0,
    }
}

pub fn set_direction(gpio_num: u8, direction: Directions) {
    let mut filepath = String::from("/sys/class/gpio/gpio%/direction");
    let gpio = gpio_num.to_string();
    filepath = filepath.replace('%', gpio.as_str());

    let mut file: File = match open_file(&filepath) {
        Err(why) => panic!("couldn't open {}: {}", filepath, why),
        Ok(f) => f,
    };

    if let Err(why) = file.write_all(direction.as_bytes()) {
        panic!("couldn't write to {}: {}", filepath, why);
    }
}
// TODO??: test something here??
