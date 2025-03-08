use ssh2::Session;
use std::io::{BufRead, BufReader, Read};
use std::net::TcpStream;
use std::path::Path;
use std::thread;
use std::time::Duration;

pub fn run_over_ssh(
    host: &str,
    port: u16,
    username: &str,
    password: Option<&str>,
    ssh_key_path: Option<&str>,
    command: &str,
) -> Result<String, String> {
    // Connect to the remote server
    let tcp = TcpStream::connect((host, port)).map_err(|e| e.to_string())?;
    let mut session = Session::new().map_err(|e| e.to_string())?;
    session.set_tcp_stream(tcp);
    session.handshake().map_err(|e| e.to_string())?;

    // Authenticate using either SSH key or password
    if let Some(key_path) = ssh_key_path {
        // Use SSH key for authentication
        session
            .userauth_pubkey_file(username, None, Path::new(key_path), None)
            .map_err(|e| e.to_string())?;
    } else if let Some(pass) = password {
        // Use password for authentication
        session
            .userauth_password(username, pass)
            .map_err(|e| e.to_string())?;
    } else {
        return Err("Either password or SSH key path must be provided".to_string());
    }

    // Ensure the session is authenticated
    if !session.authenticated() {
        return Err("Authentication failed".to_string());
    }

    // Execute the command
    let mut channel = session.channel_session().map_err(|e| e.to_string())?;
    channel.exec(command).map_err(|e| e.to_string())?;

    // Read the output of the command
    let mut output = String::new();
    channel
        .read_to_string(&mut output)
        .map_err(|e| e.to_string())?;

    // Close the channel and session
    channel.wait_close().map_err(|e| e.to_string())?;
    let exit_status = channel.exit_status().map_err(|e| e.to_string())?;

    if exit_status != 0 {
        return Err(format!("Command failed with exit status: {}", exit_status));
    }

    Ok(output)
}

pub fn journal_over_ssh(
    host: &str,
    port: u16,
    username: &str,
    password: Option<&str>,
    ssh_key_path: Option<&str>,
    servicename: &str,
) -> Result<(), String> {
    let command = &format!("sudo journalctl -u {} -f", servicename);
    // Connect to the remote server
    let tcp = TcpStream::connect((host, port)).map_err(|e| e.to_string())?;
    let mut session = Session::new().map_err(|e| e.to_string())?;
    session.set_tcp_stream(tcp);
    session.handshake().map_err(|e| e.to_string())?;

    // Authenticate using either SSH key or password
    if let Some(key_path) = ssh_key_path {
        session
            .userauth_pubkey_file(username, None, Path::new(key_path), None)
            .map_err(|e| e.to_string())?;
    } else if let Some(pass) = password {
        session
            .userauth_password(username, pass)
            .map_err(|e| e.to_string())?;
    } else {
        return Err("Either password or SSH key path must be provided".to_string());
    }

    // Ensure the session is authenticated
    if !session.authenticated() {
        return Err("Authentication failed".to_string());
    }

    // Open a channel and execute the command
    let mut channel = session.channel_session().map_err(|e| e.to_string())?;
    channel.exec(command).map_err(|e| e.to_string())?;

    // Read the command output line by line
    let reader = BufReader::new(channel.stream(0)); // stdout stream
    for line in reader.lines() {
        match line {
            Ok(log_line) => println!("{}", log_line),
            Err(_) => break, // Stop on read error (e.g., connection closed)
        }
        thread::sleep(Duration::from_millis(100)); // Prevent excessive CPU usage
    }

    // Ensure the command and channel close properly
    channel.wait_close().map_err(|e| e.to_string())?;
    let exit_status = channel.exit_status().map_err(|e| e.to_string())?;

    if exit_status != 0 {
        return Err(format!("Command failed with exit status: {}", exit_status));
    }

    Ok(())
}
