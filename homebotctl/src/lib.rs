use ssh2::Session;
use std::fs::File;
use std::io::prelude::*;
use std::net::TcpStream;
use std::path::Path;
use std::process::Command;
use std::process::ExitStatus;

pub mod cfg;

pub fn run_local_command(command: &str) {
    // Split the command into program and arguments
    let mut parts = command.split_whitespace();
    let program = parts.next().expect("No program specified");
    let args: Vec<&str> = parts.collect();

    // Create a new Command instance
    let mut cmd = Command::new(program);
    cmd.args(args);

    println!("Running: {}", command);

    // Execute the command and handle the result
    let status: ExitStatus = cmd
        .status()
        .expect(&format!("Failed to execute '{}'", command));

    // Check if the command succeeded
    if !status.success() {
        eprintln!("'{}' failed with exit code: {:?}", command, status.code());
    }
}

pub fn run_over_ssh(
    host: &str,
    port: u16,
    username: &str,
    password: &str,
    command: &str,
) -> Result<String, String> {
    // Connect to the remote server
    let tcp = TcpStream::connect((host, port)).map_err(|e| e.to_string())?;
    let mut session = Session::new().map_err(|e| e.to_string())?;
    session.set_tcp_stream(tcp);
    session.handshake().map_err(|e| e.to_string())?;

    // Authenticate with username and password
    session
        .userauth_password(username, password)
        .map_err(|e| e.to_string())?;

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

pub fn run_cargo_build(
    path: &str,
    features: Option<String>,
    release: bool,
    target: Option<String>,
) {
    let mut args = vec!["build".to_string()];

    // Add --features if specified
    if let Some(features) = features {
        args.push("--features".to_string());
        args.push(features);
    }

    // Add --release if specified
    if release {
        args.push("--release".to_string());
    }

    // Add --target if specified
    if let Some(target) = target {
        args.push("--target".to_string());
        args.push(target);
    }

    // Convert Vec<String> to Vec<&str> for Command::args
    let args_ref: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
    run_cargo_command(path, "cargo", &args_ref);
}

pub fn run_cargo_command(path: &str, command: &str, args: &[&str]) {
    // Check if the directory exists
    if !Path::new(path).exists() {
        eprintln!("Error: Directory '{}' does not exist.", path);
        return;
    }

    // Run the cargo command
    let mut cmd = Command::new(command);
    cmd.args(args).current_dir(path);

    println!("Running: {} {} in {}", command, args.join(" "), path);

    let status = cmd
        .status()
        .expect(&format!("Failed to execute '{}'", command));

    // Check if the command succeeded
    if !status.success() {
        eprintln!(
            "'{} {}' failed with exit code: {:?}",
            command,
            args.join(" "),
            status.code()
        );
    }
}

pub fn copy_file_over_ssh(
    host: &str,
    port: u16,
    username: &str,
    password: &str,
    local_file_path: &str,
    remote_file_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Connect to the remote server
    let tcp = TcpStream::connect((host, port))?;
    let mut sess = Session::new()?;
    sess.set_tcp_stream(tcp);
    sess.handshake()?;
    sess.userauth_password(username, password)?;

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
