use crate::comms::*;

use crate::gpio_robot::*;
use load_dotenv::load_dotenv;
use log::{debug, info};
use std::collections::HashMap;
use std::sync::mpsc;
use std::thread;

pub struct LedActionServerNode<'a> {
    port_in: &'a str,
    conns: HashMap<&'a str, &'a str>,
    led: GPIOLed,
    led_on: bool,
}

impl<'a> LedActionServerNode<'a> {
    pub fn new(name: &'a str, conns: HashMap<&'a str, &'a str>) -> Self {
        load_dotenv!(); //TODO??: is it better to pass parameters when needed?
        let led = GPIOLed::new(env!("LEDMAIN_PIN").parse::<u8>().unwrap());
        let node = match get_port(name, conns.clone()) {
            Ok(c) => LedActionServerNode {
                port_in: c,
                conns,
                led,
                led_on: false,
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
        let status_node = get_port("status", self.conns.clone()).unwrap();
        let comms = UDPComms::new(self.port_in.to_owned());
        let _status: HashMap<String, String> = HashMap::new();
        let (tx, rx) = mpsc::channel();
        loop {
            //println!("led LOOP");
            let mut this_comms = comms.clone();
            let this_tx = tx.clone();
            let h = thread::spawn(move || {
                this_comms.get_data(this_tx);
            });

            let rcvd = rx.recv().unwrap();
            debug!("[led] Received: {}", rcvd);
            if rcvd == "SET:on" {
                info!("[led] Setting LED ON");
                self.led_on = true;
                self.led.on();
                comms.send_to("SET:led:on".as_bytes(), status_node);
            } else if rcvd == "SET:off" {
                info!("[led] Setting LED OFF");
                self.led_on = false;
                self.led.off();
                comms.send_to("SET:led:off".as_bytes(), status_node);
            } else if rcvd == "SET:switch" {
                if self.led_on {
                    info!("[led] Setting LED OFF");
                    self.led_on = false;
                    self.led.off();
                    comms.send_to("SET:led:off".as_bytes(), status_node);
                } else {
                    info!("[led] Setting LED ON");
                    self.led_on = true;
                    self.led.on();
                    comms.send_to("SET:led:on".as_bytes(), status_node);
                }
            }
            h.join().unwrap();
            //println!("led LOOP END");
        }
    }
}

#[cfg(test)]
mod led_node_tests {
    use super::*;
    #[test]
    fn test_object_created_ok() {
        use crate::get_conns;
        let _test_node1 = LedActionServerNode::new(
            "led",
            get_conns(["motor_l", "motor_r", "led", "status"].to_vec()),
        );
        let _test_node2 = LedActionServerNode::new("led", get_conns(["led"].to_vec()));
    }
    #[test]
    #[should_panic]
    fn test_object_created_not_ok() {
        use crate::get_conns;
        let _test_node1 =
            LedActionServerNode::new("led", get_conns(["motor_l", "motor_r", "status"].to_vec()));
        let _test_node2 = LedActionServerNode::new("led", get_conns(["status"].to_vec()));
    }

    //TODO??: test talk, but how??
}
