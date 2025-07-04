use std::time::{Duration, SystemTime};

use crate::error::*;
#[cfg(any(feature = "test", feature = "live"))]
use crate::hw_live::*;
#[cfg(feature = "sim")]
use crate::hw_sim::*;

#[derive(Debug, Clone)]
pub struct Input {
    time_step: i32,
    ts_start: SystemTime,
    ts: Duration,
    serial_port: String,
    sensor_ids: Vec<u16>,
    distance: Vec<f64>,
}

impl Input {
    pub async fn init(time_step: i32) -> Result<Self, AppError> {
        let ts_start: SystemTime = SystemTime::now();
        robot_init();
        let (serial_port, sensor_ids) = get_serial_port(time_step).await?;
        Ok(Self {
            time_step,
            ts_start,
            ts: Duration::from_millis(0),
            serial_port,
            sensor_ids,
            distance: [0.00].to_vec(),
        })
    }

    pub async fn update(&mut self) -> (Duration, Vec<f64>) {
        self.ts = self
            .ts_start
            .elapsed()
            .expect("Error retrieving time since start");

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
    /*
        pub fn react(&self, log: Log) -> Vec<CAction> {
            let mut result = vec![];
            let ta = self.isit_turnaround();
            let mo = self.isit_moveon();
            log.debug(&format!("----{:#?}", self.distance[0]));
            log.debug(&format!("TA: {:#?}, MO: {:#?}", ta.len(), mo.len()));
            for ca in ta {
                log.info(&format!("-TA"));
                result.push(ca);
            }
            for ca in mo {
                log.info(&format!("-MO"));
                result.push(ca);
            }
            result
        }

        pub fn isit_turnaround(&self) -> Vec<CAction> {
            let mut yes_itis = false;
            let mut result = vec![];
            for dist in &self.distance {
                if *dist < 1500.0 {
                    yes_itis = true;
                };
            }
            if yes_itis {
                //result.push(get_turnaround_reactionset(self.ts.clone()));
                result.push(get_base_reactionset(self.ts.clone()));
            };
            return result;
        }

        pub fn isit_moveon(&self) -> Vec<CAction> {
            let mut yes_itis = true;
            let mut result = vec![];
            for dist in &self.distance {
                if *dist < 1500.0 {
                    yes_itis = false;
                };
            }
            if yes_itis {
                //result.push(get_moveon_reactionset(self.ts.clone()));
                result.push(get_base_reactionset(self.ts.clone()));
            };
            return result;
        }
    */
}
