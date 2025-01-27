// thanks to https://github.com/tmsdev82/rust-warp-rest-api-tutorial
use crate::comms::*;

use load_dotenv::load_dotenv;
use std::sync::mpsc;
use std::thread;
use std::{collections::HashMap, convert::Infallible, sync::Arc};
use tokio::sync::Mutex;
use warp::{http::StatusCode, reply, Filter, Rejection, Reply};

type ItemNode = Arc<Mutex<String>>;
type ItemComms<'a> = Arc<Mutex<UDPComms<'a>>>;
type Result<T> = std::result::Result<T, Rejection>;

pub struct ApiNode<'a> {
    port_in: &'a str,
    port_api: u16,
    conns: HashMap<&'a str, &'a str>,
}

impl<'a> ApiNode<'a> {
    pub fn new(name: &'a str, conns: HashMap<&'a str, &'a str>) -> Self {
        load_dotenv!(); //TODO??: is it better to pass parameters when needed?
        let p_api = env!("APIPORT").parse::<u16>().unwrap();
        let node = match get_port(name, conns.clone()) {
            Ok(c) => ApiNode {
                port_in: c,
                port_api: p_api,
                conns,
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
    pub async fn talk(&mut self) {
        let mut nodes: HashMap<String, String> = HashMap::new();
        nodes.insert(
            "status".to_owned(),
            get_port("status", self.conns.clone()).unwrap().to_owned(),
        );
        nodes.insert(
            "led".to_owned(),
            get_port("led", self.conns.clone()).unwrap().to_owned(),
        );
        nodes.insert(
            "motor_l".to_owned(),
            get_port("motor_l", self.conns.clone()).unwrap().to_owned(),
        );
        nodes.insert(
            "motor_r".to_owned(),
            get_port("motor_r", self.conns.clone()).unwrap().to_owned(),
        );
        let comms = UDPComms::new(self.port_in.to_owned());
        run(self.port_api, nodes, comms).await;
    }
}

async fn run(port_api: u16, nodes: HashMap<String, String>, comms_orig: UDPComms<'static>) {
    // TODO??: use the same logging format, pass the log level to both - is this possible?
    // TODO**: use proper API actions, Get, Post...only when it makes sense -> first app must be ready
    let status: ItemNode = Arc::new(Mutex::new(nodes.get("status").unwrap().to_string()));
    let led_node: ItemNode = Arc::new(Mutex::new(nodes.get("led").unwrap().to_string()));
    let motor_l_node: ItemNode = Arc::new(Mutex::new(nodes.get("motor_l").unwrap().to_string()));
    let motor_r_node: ItemNode = Arc::new(Mutex::new(nodes.get("motor_r").unwrap().to_string()));
    let comms: ItemComms = Arc::new(Mutex::new(comms_orig));
    let root = warp::path::end().map(|| "Welcome to my warp server!");
    let get_empty_route = warp::path("get")
        .and(warp::post())
        .and(with_node(led_node.clone()))
        .and(with_comms(comms.clone()))
        .and_then(get_empty);

    let get_status_route = warp::path!("get" / "status" / ..)
        .and(warp::get())
        .and(with_node(status.clone()))
        .and(with_comms(comms.clone()))
        .and_then(get_status);

    let get_mode_route = warp::path!("get" / "mode" / ..)
        .and(warp::post())
        .and(with_node(led_node.clone()))
        .and(with_comms(comms.clone()))
        .and_then(get_mode);

    let do_scan_route = warp::path!("do" / "scan" / ..)
        .and(warp::post())
        .and(with_node(led_node.clone()))
        .and(with_comms(comms.clone()))
        .and_then(do_scan);

    let do_stop_route = warp::path!("do" / "stop" / ..)
        .and(warp::post())
        .and(with_node(led_node.clone()))
        .and(with_node(motor_l_node.clone()))
        .and(with_node(motor_r_node.clone()))
        .and(with_comms(comms.clone()))
        .and_then(do_stop);

    let do_fwd_route = warp::path!("do" / "fwd" / ..)
        .and(warp::post())
        .and(with_node(led_node.clone()))
        .and(with_node(motor_l_node.clone()))
        .and(with_node(motor_r_node.clone()))
        .and(with_comms(comms.clone()))
        .and_then(do_fwd);

    let do_bwd_route = warp::path!("do" / "bwd" / ..)
        .and(warp::post())
        .and(with_node(led_node.clone()))
        .and(with_node(motor_l_node.clone()))
        .and(with_node(motor_r_node.clone()))
        .and(with_comms(comms.clone()))
        .and_then(do_bwd);

    let do_left_route = warp::path!("do" / "left" / ..)
        .and(warp::post())
        .and(with_node(led_node.clone()))
        .and(with_node(motor_l_node.clone()))
        .and(with_node(motor_r_node.clone()))
        .and(with_comms(comms.clone()))
        .and_then(do_left);

    let do_right_route = warp::path!("do" / "right" / ..)
        .and(warp::post())
        .and(with_node(led_node.clone()))
        .and(with_node(motor_l_node.clone()))
        .and(with_node(motor_r_node.clone()))
        .and(with_comms(comms.clone()))
        .and_then(do_right);

    let do_led_on_route = warp::path!("do" / "led_on" / ..) // TODO??: maybe create /led/on ?
        .and(warp::post())
        .and(with_node(led_node.clone()))
        .and(with_comms(comms.clone()))
        .and_then(do_led_on);

    let do_led_off_route = warp::path!("do" / "led_off" / ..) // TODO??: maybe create /led/off ?
        .and(warp::post())
        .and(with_node(led_node.clone()))
        .and(with_comms(comms.clone()))
        .and_then(do_led_off);

    let do_led_switch_route = warp::path!("do" / "led_switch" / ..) // TODO??: maybe create /led/switch ?
        .and(warp::post())
        .and(with_node(led_node.clone()))
        .and(with_comms(comms.clone()))
        .and_then(do_led_switch);

    let routes = root
        .or(get_status_route)
        .or(get_mode_route)
        .or(get_empty_route)
        .or(do_scan_route)
        .or(do_stop_route)
        .or(do_fwd_route)
        .or(do_bwd_route)
        .or(do_left_route)
        .or(do_right_route)
        .or(do_led_on_route)
        .or(do_led_off_route)
        .or(do_led_switch_route)
        .with(warp::cors().allow_any_origin());

    warp::serve(routes).run(([0, 0, 0, 0], port_api)).await;
}

fn with_node(
    node_name: ItemNode,
) -> impl Filter<Extract = (ItemNode,), Error = Infallible> + Clone {
    warp::any().map(move || node_name.clone())
}

fn with_comms(comms: ItemComms) -> impl Filter<Extract = (ItemComms,), Error = Infallible> + Clone {
    warp::any().map(move || comms.clone())
}

pub async fn get_empty(_led_n: ItemNode, _comms_orig: ItemComms<'_>) -> Result<impl Reply> {
    let result = "";
    Ok(reply::with_status(reply::json(&result), StatusCode::OK))
}

pub async fn get_status(status_n: ItemNode, comms_orig: ItemComms<'static>) -> Result<impl Reply> {
    let status = status_n.lock().await;
    let mut this_comms = comms_orig.lock().await.clone();
    this_comms.send_to("GET:.*".as_bytes(), &status);
    let (tx, rx) = mpsc::channel();
    let h = thread::spawn(move || {
        this_comms.get_data(tx);
    });
    let (rcvd, _) = remove_sender(&rx.recv().unwrap());
    h.join().unwrap();
    Ok(reply::with_status(reply::json(&rcvd), StatusCode::OK))
}

pub async fn get_mode(led_n: ItemNode, comms_orig: ItemComms<'_>) -> Result<impl Reply> {
    let led = led_n.lock().await;
    comms_orig
        .lock()
        .await
        .send_to("SET:led:on".as_bytes(), &led);
    let result = " get mode".to_owned();
    Ok(reply::with_status(reply::json(&result), StatusCode::OK))
}

pub async fn do_scan(_led_n: ItemNode, _comms_orig: ItemComms<'_>) -> Result<impl Reply> {
    let result = "";
    Ok(reply::with_status(reply::json(&result), StatusCode::OK))
}

pub async fn do_stop(
    _led_n: ItemNode,
    motor_l_n: ItemNode,
    motor_r_n: ItemNode,
    comms_orig: ItemComms<'_>,
) -> Result<impl Reply> {
    let motor_l = motor_l_n.lock().await;
    comms_orig
        .lock()
        .await
        .send_to("SET:stp".as_bytes(), &motor_l);
    let motor_r = motor_r_n.lock().await;
    comms_orig
        .lock()
        .await
        .send_to("SET:stp".as_bytes(), &motor_r);
    let result = "";
    Ok(reply::with_status(reply::json(&result), StatusCode::OK))
}

pub async fn do_fwd(
    _led_n: ItemNode,
    motor_l_n: ItemNode,
    motor_r_n: ItemNode,
    comms_orig: ItemComms<'_>,
) -> Result<impl Reply> {
    let motor_l = motor_l_n.lock().await;
    comms_orig
        .lock()
        .await
        .send_to("SET:fwd".as_bytes(), &motor_l);
    let motor_r = motor_r_n.lock().await;
    comms_orig
        .lock()
        .await
        .send_to("SET:fwd".as_bytes(), &motor_r);
    let result = "";
    Ok(reply::with_status(reply::json(&result), StatusCode::OK))
}

pub async fn do_bwd(
    _led_n: ItemNode,
    motor_l_n: ItemNode,
    motor_r_n: ItemNode,
    comms_orig: ItemComms<'_>,
) -> Result<impl Reply> {
    let motor_l = motor_l_n.lock().await;
    comms_orig
        .lock()
        .await
        .send_to("SET:bwd".as_bytes(), &motor_l);
    let motor_r = motor_r_n.lock().await;
    comms_orig
        .lock()
        .await
        .send_to("SET:bwd".as_bytes(), &motor_r);
    let result = "";
    Ok(reply::with_status(reply::json(&result), StatusCode::OK))
}

pub async fn do_left(
    _led_n: ItemNode,
    motor_l_n: ItemNode,
    motor_r_n: ItemNode,
    comms_orig: ItemComms<'_>,
) -> Result<impl Reply> {
    let motor_l = motor_l_n.lock().await;
    comms_orig
        .lock()
        .await
        .send_to("SET:bwd".as_bytes(), &motor_l);
    let motor_r = motor_r_n.lock().await;
    comms_orig
        .lock()
        .await
        .send_to("SET:fwd".as_bytes(), &motor_r);
    let result = "";
    Ok(reply::with_status(reply::json(&result), StatusCode::OK))
}

pub async fn do_right(
    _led_n: ItemNode,
    motor_l_n: ItemNode,
    motor_r_n: ItemNode,
    comms_orig: ItemComms<'_>,
) -> Result<impl Reply> {
    let motor_l = motor_l_n.lock().await;
    comms_orig
        .lock()
        .await
        .send_to("SET:fwd".as_bytes(), &motor_l);
    let motor_r = motor_r_n.lock().await;
    comms_orig
        .lock()
        .await
        .send_to("SET:bwd".as_bytes(), &motor_r);
    let result = "";
    Ok(reply::with_status(reply::json(&result), StatusCode::OK))
}

pub async fn do_led_on(led_n: ItemNode, comms_orig: ItemComms<'_>) -> Result<impl Reply> {
    let led = led_n.lock().await;
    comms_orig.lock().await.send_to("SET:on".as_bytes(), &led);
    let result = "";
    Ok(reply::with_status(reply::json(&result), StatusCode::OK))
}

pub async fn do_led_off(led_n: ItemNode, comms_orig: ItemComms<'_>) -> Result<impl Reply> {
    let led = led_n.lock().await;
    comms_orig.lock().await.send_to("SET:off".as_bytes(), &led);
    let result = "";
    Ok(reply::with_status(reply::json(&result), StatusCode::OK))
}

pub async fn do_led_switch(led_n: ItemNode, comms_orig: ItemComms<'_>) -> Result<impl Reply> {
    let led = led_n.lock().await;
    comms_orig
        .lock()
        .await
        .send_to("SET:switch".as_bytes(), &led);
    let result = "";
    Ok(reply::with_status(reply::json(&result), StatusCode::OK))
}

#[cfg(test)]
mod api_node_tests {
    use super::*;
    #[test]
    fn test_object_created_ok() {
        use crate::get_conns;
        let _test_node1 = ApiNode::new(
            "api",
            get_conns(["motor_l", "motor_r", "api", "led", "status"].to_vec()),
        );
        let _test_node2 = ApiNode::new("api", get_conns(["api"].to_vec()));
    }
    #[test]
    #[should_panic]
    fn test_object_created_not_ok() {
        use crate::get_conns;
        let _test_node1 = ApiNode::new("api", get_conns(["motor_l", "motor_r", "status"].to_vec()));
        let _test_node2 = ApiNode::new("api", get_conns(["status"].to_vec()));
    }

    // TODO??: test each call?
}
