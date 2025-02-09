use ssh2::Session;
use std::fs::File;
use std::io::prelude::*;
use std::net::TcpStream;
use std::path::Path;
use std::process::Command;

pub fn run_local_command(command: &str) {
    let mut cmd = Command::new(command);

    println!("Running: {}", command);

    let status = cmd
        .status()
        .expect(&format!("Failed to execute '{}'", command));

    // Check if the command succeeded
    if !status.success() {
        eprintln!("'{}' failed with exit code: {:?}", command, status.code());
    }
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

pub fn run_over_ssh(
    host: &str,
    port: u16,
    username: &str,
    password: &str,
    local_file_path: &str,
    remote_file_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // TODO: this and test
    return Ok(());
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
