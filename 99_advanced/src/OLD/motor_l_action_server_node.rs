use crate::comms::*;

use crate::gpio_robot::*;
use load_dotenv::load_dotenv;
use log::{debug, info};
use std::collections::HashMap;
use std::sync::mpsc;
use std::thread;

pub struct MotorLActionServerNode<'a> {
    port_in: &'a str,
    conns: HashMap<&'a str, &'a str>,
    motor: GPIOMotor,
    speed: i8,
}

impl<'a> MotorLActionServerNode<'a> {
    pub fn new(name: &'a str, conns: HashMap<&'a str, &'a str>) -> Self {
        load_dotenv!(); //TODO??: is it better to pass parameters when needed?
        let motor = GPIOMotor::new(
            env!("MOTOR_L_PIN_IN1").parse::<u8>().unwrap(),
            env!("MOTOR_L_PIN_IN2").parse::<u8>().unwrap(),
            env!("MOTOR_L_PIN_ENA").parse::<u8>().unwrap(),
        );
        let node = match get_port(name, conns.clone()) {
            Ok(c) => MotorLActionServerNode {
                port_in: c,
                conns,
                motor,
                speed: 0,
            },
            Err(_) => {
                panic!(
                    "couldn't find a port to itself: {} (HINT: check name at main.rs)",
                    name
                );
            }
        };
        node
    }

    pub fn talk(&mut self) {
        // TODO: is this called status_node everywhere?
        let status_node = get_port("status", self.conns.clone()).unwrap();
        let comms = UDPComms::new(self.port_in.to_owned());

        let (tx, rx) = mpsc::channel();
        loop {
            let mut this_comms = comms.clone();
            let this_tx = tx.clone();
            let h = thread::spawn(move || {
                this_comms.get_data(this_tx);
            });

            //let rcvd = remove_sender(&rx.recv().unwrap());
            let rcvd_raw = rx.recv().unwrap();
            let (rcvd, _) = remove_sender(&rcvd_raw);
            debug!("[motor_l] Received: {}", rcvd);
            if rcvd == "SET:fwd" {
                info!("[motor_l] Setting Left Motor Forwards");
                self.speed = 1;
                self.motor.fwd();
                comms.send_to("SET:motorl:fwd".as_bytes(), status_node);
            } else if rcvd == "SET:bwd" {
                info!("[motor_l] Setting Left Motor Backwards");
                self.speed = -1;
                self.motor.bwd();
                comms.send_to("SET:motorl:bwd".as_bytes(), status_node);
            } else if rcvd == "SET:stp" {
                info!("[motor_l] Setting Left Motor to Stop");
                self.speed = 0;
                self.motor.stp();
                comms.send_to("SET:motorl:stp".as_bytes(), status_node);
            } else if rcvd == "SET:switch" {
                if self.speed == 0 {
                    info!("[motor_l] Setting Left Motor Forwards");
                    self.speed = 1;
                    self.motor.fwd();
                    comms.send_to("SET:motorl:fwd".as_bytes(), status_node);
                } else {
                    info!("[motor_l] Setting Left Motor to Stop");
                    self.speed = 0;
                    self.motor.stp();
                    comms.send_to("SET:motorl:stp".as_bytes(), status_node);
                }
            }
            h.join().unwrap();
        }
    }
}

#[cfg(test)]
mod led_node_tests {
    use super::*;
    #[test]
    fn test_object_created_ok() {
        use crate::get_conns;
        let _test_node1 = MotorLActionServerNode::new(
            "motor_l",
            get_conns(["motor_l", "motor_r", "led", "status"].to_vec()),
        );
        let _test_node2 = MotorLActionServerNode::new("motor_l", get_conns(["motor_l"].to_vec()));
    }
    #[test]
    #[should_panic]
    fn test_object_created_not_ok() {
        use crate::get_conns;
        let _test_node1 = MotorLActionServerNode::new(
            "motor_l",
            get_conns(["led", "motor_r", "status"].to_vec()),
        );
        let _test_node2 = MotorLActionServerNode::new("motor_l", get_conns(["status"].to_vec()));
    }

    //TODO??: test talk, but how??
}
