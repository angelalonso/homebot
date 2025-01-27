use std::collections::HashMap;
use std::net;
use std::sync::mpsc::Sender;

// This one is used by all nodes using comms, so for now it stays here
pub fn get_port<'a>(
    name: &'a str,
    conns: HashMap<&'a str, &'a str>,
) -> Result<&'a str, &'static str> {
    let res = match conns.get(name) {
        Some(r) => Ok(*r),
        None => Err("Nothing found."),
    };
    res
}

//#[derive(Debug, Clone, Copy)]
#[derive(Debug, Clone)]
pub struct UDPComms<'a> {
    ip: &'a str,
    pub rx: &'a str,
    port_in: String,
}

//impl<'a> UDPComms<'_> {
impl UDPComms<'_> {
    pub fn new(port_in: String) -> Self {
        let ip = "127.0.0.1"; // TODO??: Does this need to be on .env?
        UDPComms {
            ip,
            rx: "",
            port_in,
        }
    }

    pub fn listen(&mut self) -> String {
        let local = format!("{}:{}", self.ip, self.port_in);
        let socket = net::UdpSocket::bind(local).expect("failed to bind host socket");
        socket
            .set_read_timeout(None)
            .expect("set_read_timeout call failed");
        let mut buf = [0; 100];
        //let recvd = match socket.recv(&mut buf) {
        //    Ok(received) => format!("{}", String::from_utf8_lossy(&buf[..received]).to_owned()),
        //    Err(_) => "".to_owned(),
        //};
        ////println!("received at {} --> {}", self.port_in, recvd);
        //recvd
        match socket.recv(&mut buf) {
            Ok(received) => String::from_utf8_lossy(&buf[..received]).into_owned(),
            Err(_) => "".to_owned(),
        }
    }

    pub fn get_data(&mut self, callback: Sender<String>) {
        let local = format!("{}:{}", self.ip, self.port_in);
        let socket = net::UdpSocket::bind(local).expect("failed to bind host socket");
        socket
            .set_read_timeout(None)
            .expect("set_read_timeout call failed");
        let mut buf = [0; 100];
        let _result = match socket.recv(&mut buf) {
            Ok(received) => callback.send(format!("{}", String::from_utf8_lossy(&buf[..received]))),
            Err(_) => Ok(()),
        };
    }

    pub fn send_to(&self, raw_msg: &[u8], dest_port: &str) {
        let msg = add_sender(raw_msg, self.port_in.as_bytes());

        let local = format!("{}:{}", self.ip, self.port_in);
        let socket = match net::UdpSocket::bind(&local) {
            Ok(s) => Some(s),
            Err(e) => {
                println!("Error binding to {}: {}, retrying...", dest_port, e);
                std::thread::sleep(std::time::Duration::from_millis(10));
                match net::UdpSocket::bind(&local) {
                    Ok(s) => {
                        println!("{} connected!", dest_port);
                        Some(s)
                    }
                    Err(e) => {
                        println!("Error binding to {}: {}, retrying #2...", dest_port, e);
                        std::thread::sleep(std::time::Duration::from_millis(20));
                        match net::UdpSocket::bind(&local) {
                            Ok(s) => {
                                println!("{} connected!", dest_port);
                                Some(s)
                            }
                            Err(e) => {
                                println!("Error binding to {}: {}, retrying #3...", dest_port, e);
                                std::thread::sleep(std::time::Duration::from_millis(40));
                                match net::UdpSocket::bind(&local) {
                                    Ok(s) => {
                                        println!("{} connected!", dest_port);
                                        Some(s)
                                    }
                                    Err(_) => None,
                                }
                            }
                        }
                    }
                }
            }
        };
        match socket {
            Some(s) => {
                s.set_read_timeout(None)
                    .expect("set_read_timeout call failed");
                let destination = format!("{}:{}", self.ip, dest_port);
                //println!(
                //    "Transmitting data from {} to {} --> {:#?}",
                //    self.port_in,
                //    destination,
                //    String::from_utf8_lossy(msg)
                //);
                let _result = s
                    .send_to(&msg, destination)
                    .expect("failed to send message");

                //result
            }
            None => println!("Error using {}", dest_port),
        }
    }
}

fn add_sender<'a>(text: &'a [u8], port_from: &'a [u8]) -> Vec<u8> {
    let step_1 = [text, "|".as_bytes()].concat();
    [step_1, port_from.to_owned()].concat()
}

pub fn remove_sender(text: &'_ str) -> (String, String) {
    let result: Vec<&str> = text.split('|').collect();
    (result[0].to_string(), result[1].to_string())
}

#[cfg(test)]
mod comms_tests {
    use super::*;
    #[test]
    fn test_get_port() {
        let mut conns: HashMap<&str, &str> = HashMap::new();
        conns.insert("test", "0001");
        // expected port
        let name = "test";
        match get_port(name, conns.clone()) {
            Ok(a) => assert_eq!(a, "0001"),
            Err(e) => assert_eq!(e, "Nothing found."),
        }
        // expected error
        let name = "test2";
        match get_port(name, conns) {
            Ok(a) => assert_eq!(a, "0001"),
            Err(e) => assert_eq!(e, "Nothing found."),
        }
    }

    #[test]
    fn test_add_sender() {
        assert_eq!(
            add_sender("hello world".as_bytes(), "0001".as_bytes()),
            "hello world|0001".as_bytes()
        );
    }

    #[test]
    fn test_remove_sender() {
        assert_eq!(
            remove_sender("hello world|0001"),
            ("hello world".to_owned(), "0001".to_owned())
        );
    }
    // TODO??: test the other ones?
}
