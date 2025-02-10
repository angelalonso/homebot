use homebotctl::*;
use std::fs;

// Local Command
// ----------------
#[test]
fn test_run_local_command_success() {
    // Test creating a directory
    let dir_name = "test_dir";
    run_local_command(&format!("mkdir {}", dir_name));

    // Check if the directory was created
    assert!(
        fs::metadata(dir_name).is_ok(),
        "Directory should have been created"
    );

    // Clean up: remove the directory
    fs::remove_dir(dir_name).expect("Failed to clean up test directory");
}

#[test]
fn test_run_local_command_failure() {
    // Test running a non-existent command
    let result = std::panic::catch_unwind(|| {
        run_local_command("nonexistent_command");
    });

    // Ensure the function panics (since the command doesn't exist)
    assert!(
        result.is_err(),
        "Running a nonexistent command should panic"
    );
}

#[test]
fn test_run_local_command_with_args() {
    // Test creating a file using `touch` (assuming `touch` is available)
    let file_name = "test_file.txt";
    run_local_command(&format!("touch {}", file_name));

    // Check if the file was created
    assert!(
        fs::metadata(file_name).is_ok(),
        "File should have been created"
    );

    // Clean up: remove the file
    fs::remove_file(file_name).expect("Failed to clean up test file");
}

#[test]
fn test_run_local_command_empty_command() {
    // Test running an empty command
    let result = std::panic::catch_unwind(|| {
        run_local_command("");
    });

    // Ensure the function panics (since no program is specified)
    assert!(result.is_err(), "Running an empty command should panic");
}

// Command over SSH
// ----------------
#[test]
fn test_run_ssh_command_fail() {
    // Replace these with your SSH server details
    let host = "your.ssh.server.com";
    let port = 22;
    let username = "your_username";
    let password = "your_password";
    let command = "echo Hello, SSH!";

    // Run the SSH command
    let result = run_over_ssh(host, port, username, password, command);

    // Check if the command was successful
    match result {
        Ok(output) => {
            panic!("SSH command SHOULD HAVE failed: {}", output);
        }
        Err(e) => {
            println!("Command output: {}", e);
            assert!(e.contains("failed to lookup address information"));
        }
    }
}
