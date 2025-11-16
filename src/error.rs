use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    Gpio(gpio_cdev::Error),
    Serial(std::io::Error),
    SerialPort(tokio_serial::Error),      // Add this
    Timeout(tokio::time::error::Elapsed), // And this
    Config(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::Gpio(e) => write!(f, "GPIO error: {}", e),
            AppError::Serial(e) => write!(f, "Serial IO error: {}", e),
            AppError::SerialPort(e) => write!(f, "Serial port error: {}", e),
            AppError::Timeout(e) => write!(f, "Timeout error: {}", e),
            AppError::Config(e) => write!(f, "Config error: {}", e),
        }
    }
}

impl Error for AppError {}

impl From<gpio_cdev::Error> for AppError {
    fn from(err: gpio_cdev::Error) -> Self {
        AppError::Gpio(err)
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::Serial(err)
    }
}

impl From<tokio_serial::Error> for AppError {
    fn from(err: tokio_serial::Error) -> Self {
        AppError::SerialPort(err)
    }
}

impl From<tokio::time::error::Elapsed> for AppError {
    fn from(err: tokio::time::error::Elapsed) -> Self {
        AppError::Timeout(err)
    }
}
