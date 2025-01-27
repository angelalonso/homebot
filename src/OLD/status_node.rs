use crate::comms::*;

use chrono::Local;
use log::debug;
use regex::Regex;
use std::collections::HashMap;
use std::sync::mpsc;
use std::thread;

pub struct StatusNode<'a> {
    port_in: &'a str,
    status: HashMap<String, String>,
}

impl<'a> StatusNode<'a> {
    pub fn new(name: &'a str, conns: HashMap<&'a str, &'a str>) -> Self {
        let status: HashMap<String, String> = HashMap::new();
        let node = match get_port(name, conns.clone()) {
            Ok(c) => StatusNode { port_in: c, status },
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
        let comms = UDPComms::new(self.port_in.to_owned());
        // Add first timestamp
        let starttimestamp = Local::now().format("%Y%m%dT%H:%M:%S%.3f");
        self.set_status("starttimestamp".to_owned(), starttimestamp.to_string());
        let (tx, rx) = mpsc::channel();
        loop {
            //debug!("status LOOP");
            let this_tx = tx.clone();
            let mut this_comms = comms.clone();
            let h = thread::spawn(move || {
                this_comms.get_data(this_tx);
            });
            let rcvd_raw = rx.recv().unwrap();
            let (rcvd, call_origin) = remove_sender(&rcvd_raw);
            // IMPORTANT! We need a little bit of delay here
            std::thread::sleep(std::time::Duration::from_millis(1));
            // TODO: cleanup source of call, but also get the Port to use it later
            debug!("[status] Received: {}", rcvd);
            if rcvd.contains("SET:") {
                let split1: Vec<&str> = rcvd.split(':').collect();
                self.set_status(split1[1].to_owned(), split1[2].to_owned());
            } else if rcvd.contains("GET:") {
                let cleanedup: Vec<&str> = rcvd.split("GET:").collect();
                let split_objs: Vec<&str> = cleanedup[1].split(':').collect();
                let mut response = String::from("");
                for o in split_objs {
                    //match self.get_status_regex(o.to_owned()) {
                    //    Some(v_map) => {
                    //        for (k, v) in v_map {
                    //            let key_val_json = format!("{{ {}: {} }}", k, v);
                    //            response += &key_val_json;
                    //        }
                    //    }
                    //    None => (),
                    //};
                    if let Some(v_map) = self.get_status_regex(o.to_owned()) {
                        for (k, v) in v_map {
                            let key_val_json = format!("{{ {}: {} }}", k, v);
                            response += &key_val_json;
                        }
                    };
                }
                // TODO: test this with more values
                comms.send_to(response.as_bytes(), &call_origin);
            }
            h.join().unwrap();
        }
    }

    fn set_status(&mut self, key: String, value: String) {
        self.status.insert(key, value);
    }

    fn get_status_regex(&self, regex: String) -> Option<HashMap<String, String>> {
        let mut result: HashMap<String, String> = HashMap::new();
        let re = Regex::new(&regex).unwrap();
        for (k, v) in self.status.clone() {
            if re.is_match(&k) {
                result.insert(k, v);
            }
        }
        if !result.is_empty() {
            Some(result)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod status_node_tests {
    use super::*;
    #[test]
    fn test_object_created_ok() {
        use crate::get_conns;
        let _test_node1 = StatusNode::new(
            "status",
            get_conns(["motor_l", "motor_r", "led", "status"].to_vec()),
        );
        let _test_node2 = StatusNode::new("status", get_conns(["status"].to_vec()));
    }
    #[test]
    #[should_panic]
    fn test_object_created_not_ok() {
        use crate::get_conns;
        let _test_node1 =
            StatusNode::new("status", get_conns(["motor_l", "motor_r", "led"].to_vec()));
        let _test_node2 = StatusNode::new("status", get_conns(["led"].to_vec()));
    }

    #[test]
    fn test_regex_get() {
        use crate::get_conns;
        let mut test_node1 = StatusNode::new(
            "status",
            get_conns(["motor_l", "motor_r", "led", "status"].to_vec()),
        );
        let mut status_check: HashMap<String, String> = HashMap::new();
        test_node1.set_status("test1".to_owned(), "01".to_owned());
        status_check.insert("test1".to_owned(), "01".to_owned());
        test_node1.set_status("test2".to_owned(), "01".to_owned());
        status_check.insert("test2".to_owned(), "01".to_owned());
        test_node1.set_status("test21".to_owned(), "01".to_owned());
        status_check.insert("test21".to_owned(), "01".to_owned());
        assert_eq!(
            test_node1.get_status_regex(".*".to_owned()),
            Some(status_check)
        );
    }
    //TODO: somehow this does not update status for both motors
    // Proposal: function to update status, bomb it with several requests at once

    #[test]
    fn test_set_status() {
        use crate::get_conns;
        let mut test_node1 = StatusNode::new(
            "status",
            get_conns(["motor_l", "motor_r", "led", "status"].to_vec()),
        );
        test_node1.set_status("key1".to_owned(), "value1".to_owned());
    }

    //TODO??: test talk, but how??
}
