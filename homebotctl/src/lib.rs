use ssh2::Session;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::Read;
use std::net::{Ipv4Addr, SocketAddrV4, TcpStream};
use std::path::Path;
use std::str::FromStr;
use std::time::Duration;

pub mod cfg;
pub mod modes;
pub mod remote;
pub mod local;
pub fn is_bot_online(ip_text: &str, port: u16) -> bool {
    let ip =
        Ipv4Addr::from_str(ip_text).map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e));
    let socket_addr = SocketAddrV4::new(ip.expect("IP FORMAT IS RONG"), port);

    if TcpStream::connect_timeout(&socket_addr.into(), Duration::from_millis(100)).is_ok() {
        return true;
    }
    return false;
}

pub fn get_ips_open(base_ip: &str, subnet_mask: u32, port: u16) {
    let base_ip = Ipv4Addr::from_str(base_ip).expect("Invalid base IP address");
    let num_hosts = 2u32.pow(32 - subnet_mask);

    for i in 1..num_hosts - 1 {
        let ip = Ipv4Addr::from(u32::from(base_ip) + i);

        let socket_addr = SocketAddrV4::new(ip, port);

        if TcpStream::connect_timeout(&socket_addr.into(), Duration::from_millis(100)).is_ok() {
            println!("{}:{} is OPEN, please adapt ctl config file", ip, port);
            break;
        }
    }
}
pub fn copy_file_over_ssh(
    host: &str,
    port: u16,
    username: &str,
    password: Option<&str>,
    ssh_key_path: Option<&str>,
    local_file_path: &str,
    remote_file_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Connect to the remote server
    let tcp = TcpStream::connect((host, port))?;
    let mut sess = Session::new()?;
    sess.set_tcp_stream(tcp);
    sess.handshake()?;

    // Authenticate using either SSH key or password
    if let Some(key_path) = ssh_key_path {
        // Use SSH key for authentication
        sess.userauth_pubkey_file(username, None, Path::new(key_path), None)?;
    } else if let Some(pass) = password {
        // Use password for authentication
        sess.userauth_password(username, pass)?;
    } else {
        return Err("Either password or SSH key path must be provided".into());
    }

    // Ensure the session is authenticated
    if !sess.authenticated() {
        return Err("Authentication failed".into());
    }

    // Open the local file
    let mut file = File::open(local_file_path)?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;

    // Create a new SCP session
    let mut remote_file = sess.scp_send(
        Path::new(remote_file_path),
        0o644,
        contents.len() as u64,
        None,
    )?;

    // Write the file contents to the remote server
    remote_file.write_all(&contents)?;

    // Close the SCP session
    remote_file.send_eof()?;
    remote_file.wait_eof()?;
    remote_file.close()?;
    remote_file.wait_close()?;

    Ok(())
}
