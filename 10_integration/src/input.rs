use crate::error::*;
#[cfg(any(feature = "test", feature = "live"))]
use crate::hw_arduino::*;
#[cfg(feature = "sim")]
use crate::hw_sim::*;
use crate::loggin::Log;
use std::time::SystemTime;

use std::time::Duration;

#[derive(Debug, Clone)]
pub struct Input {
    ts_start: SystemTime,
    ts: Duration,
    in_port: String,
    distance: Vec<f64>,
}

impl Input {
    pub async fn init(time_step: i32) -> Result<Self, AppError> {
        let ts_start: SystemTime = SystemTime::now();
        robot_init();
        let in_port: String = find_port(time_step).await?;
        Ok(Self {
            ts_start,
            ts: Duration::from_millis(0),
            in_port,
            distance: [0.00].to_vec(),
        })
    }

    pub fn update(&mut self) -> Duration {
        self.ts = self
            .ts_start
            .elapsed()
            .expect("Error retrieving time since start");

        self.read_distance();

        return self.ts;
    }

    pub fn set(&mut self, ts: Duration, distance: Vec<f64>) {
        self.ts = ts;
        self.distance = distance;
    }

    pub fn set_ts(&mut self, ts: Duration) {
        self.ts = ts;
    }

    pub fn set_distance(&mut self, _log: Log, distance: Vec<f64>) {
        self.distance = distance.clone();
    }

    pub fn read_distance(&mut self) {
        // let distance_values = self
        //     .in_port
        let distance_values: Vec<_> = self.in_port_to_vec()
            .iter()
            .map(|sensor| distance_sensor_get_value(*sensor))
            .collect();
        self.distance = distance_values.clone();
    }

    pub fn get_ts(&mut self) -> Duration {
        self.ts
    }

    pub fn get_distance(&mut self) -> Vec<f64> {
        self.distance.clone()
    }
    // needed for webots
    pub fn in_port_to_vec(&self) -> Vec<u16> {
        let current = &self.in_port;
        let restored_vec: Vec<u16> = current.encode_utf16().collect();
        restored_vec
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
