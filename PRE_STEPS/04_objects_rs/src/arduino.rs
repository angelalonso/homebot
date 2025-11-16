use crate::error::AppError;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::time::{timeout, Duration};
use tokio_serial::{SerialPortBuilderExt, SerialStream};

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

pub async fn find_arduino() -> Result<String, AppError> {
    let ports = tokio_serial::available_ports()?;
    ports
        .into_iter()
        .find(|p| p.port_name.contains("ACM") || p.port_name.contains("USB"))
        .map(|p| p.port_name)
        .ok_or_else(|| AppError::Config("No Arduino found".into()))
}
