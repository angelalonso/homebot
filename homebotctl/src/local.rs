use std::process::Command;
use std::process::ExitStatus;
use std::path::Path;

//pub fn run_cargo_command(path: &str, command: &str, args: &[&str]) {
//    // Check if the directory exists
//    if !Path::new(path).exists() {
//        eprintln!("Error: Directory '{}' does not exist.", path);
//        return;
//    }
//
//    // Run the cargo command
//    let mut cmd = Command::new(command);
//    cmd.args(args).current_dir(path);
//
//    println!("Running: {} {} in {}", command, args.join(" "), path);
//
//    let status = cmd
//        .status()
//        .expect(&format!("Failed to execute '{}'", command));
//
//    // Check if the command succeeded
//    if !status.success() {
//        eprintln!(
//            "'{} {}' failed with exit code: {:?}",
//            command,
//            args.join(" "),
//            status.code()
//        );
//    }
//}

pub fn run_cargo_command(path: &str, command: &str, args: &[&str]) -> Result<String, String> {
    // Check if the directory exists
    if !Path::new(path).exists() {
        return Err(format!("Error: Directory '{}' does not exist.", path));
    }

    // Run the cargo command
    let mut cmd = Command::new(command);
    cmd.args(args).current_dir(path);

    println!("Running: {} {} in {}", command, args.join(" "), path);

    let status = cmd
        .status()
        .map_err(|e| format!("Failed to execute '{}': {}", command, e))?;

    // Check if the command succeeded
    if !status.success() {
        return Err(format!(
            "'{} {}' failed with exit code: {:?}",
            command,
            args.join(" "),
            status.code()
        ));
    }

    // Return success message
    Ok(format!(
        "'{} {}' executed successfully in '{}'",
        command,
        args.join(" "),
        path
    ))
}

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
    match run_cargo_command(path, "cargo", &args_ref) {
        Ok(msg) => {
            println!("Result: {:#?}", msg);
        }
        Err(e) => {
            println!("ERROR Running Cargo Build: {:#?}", e);
        }
    };
}


