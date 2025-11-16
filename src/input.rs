use std::time::{Duration, SystemTime};

use crate::error::*;
#[cfg(any(feature = "test", feature = "live"))]
use crate::hw_live::*;
#[cfg(feature = "sim")]
use crate::hw_sim::*;

#[cfg(feature = "test")]
#[derive(Debug, Clone)]
pub struct MockInput {
    pub timestamp: Duration,
    pub distances: Vec<f64>,
    pub sensor_ids: Vec<u16>,
}

#[cfg(feature = "test")]
impl MockInput {
    pub fn new() -> Self {
        Self {
            timestamp: Duration::from_millis(0),
            distances: vec![0.0],
            sensor_ids: vec![0],
        }
    }

    pub fn set_timestamp(&mut self, ts: Duration) {
        self.timestamp = ts;
    }

    pub fn set_distances(&mut self, distances: Vec<f64>) {
        self.distances = distances;
    }

    pub fn set_sensor_ids(&mut self, sensor_ids: Vec<u16>) {
        self.sensor_ids = sensor_ids;
    }
}

#[derive(Debug, Clone)]
pub enum InputType {
    Live(Input),
    #[cfg(feature = "test")]
    Mock(MockInput),
}

#[derive(Debug, Clone)]
pub struct Input {
    time_step: i32,
    ts_start: SystemTime,
    ts: Duration,
    serial_port: String,
    sensor_ids: Vec<u16>,
    distance: Vec<f64>,
    #[cfg(feature = "test")]
    test_mode: bool,
}

impl Input {
    pub async fn init(time_step: i32) -> Result<Self, AppError> {
        let ts_start: SystemTime = SystemTime::now();
        
        #[cfg(not(feature = "test"))]
        robot_init();
        
        let (serial_port, sensor_ids) = get_serial_port(time_step).await?;
        
        Ok(Self {
            time_step,
            ts_start,
            ts: Duration::from_millis(0),
            serial_port,
            sensor_ids,
            distance: [0.00].to_vec(),
            #[cfg(feature = "test")]
            test_mode: false,
        })
    }

    #[cfg(feature = "test")]
    pub fn init_test(time_step: i32) -> Result<Self, AppError> {
        let ts_start: SystemTime = SystemTime::now();
        
        Ok(Self {
            time_step,
            ts_start,
            ts: Duration::from_millis(0),
            serial_port: "mock".to_string(),
            sensor_ids: vec![0],
            distance: [0.00].to_vec(),
            test_mode: true,
        })
    }

    pub async fn update(&mut self) -> (Duration, Vec<f64>) {
        self.ts = self
            .ts_start
            .elapsed()
            .expect("Error retrieving time since start");

        // In test mode, don't try to read from actual hardware
        #[cfg(feature = "test")]
        if self.test_mode {
            return (self.ts, self.get_distance());
        }

        let d = read_distance(&self.serial_port, self.sensor_ids.clone(), self.time_step.clone());
        self.set_distance(d.await);

        return (self.ts, self.get_distance());
    }

    pub fn set(&mut self, ts: Duration, distance: Vec<f64>) {
        self.ts = ts;
        self.distance = distance;
    }

    pub fn set_ts(&mut self, ts: Duration) {
        self.ts = ts;
    }

    pub fn set_distance(&mut self, distance: Vec<f64>) {
        self.distance = distance.clone();
    }

    pub fn get_ts(&mut self) -> Duration {
        self.ts
    }

    pub fn get_sens(&mut self) -> Vec<u16> {
        self.sensor_ids.clone()
    }

    pub fn get_distance(&mut self) -> Vec<f64> {
        self.distance.clone()
    }
    
    #[cfg(feature = "test")]
    pub fn set_test_mode(&mut self, test_mode: bool) {
        self.test_mode = test_mode;
    }
    
    #[cfg(feature = "test")]
    pub fn get_test_mode(&self) -> bool {
        self.test_mode
    }
}
