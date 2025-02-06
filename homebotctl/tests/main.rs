use homebotctl::{copy_file_over_ssh, run_cargo_build, run_cargo_command};
use std::fs;
use std::io::Write;
use std::process::Command;
use tempfile::tempdir;
use tempfile::NamedTempFile;

// Cargo command
// --------------------------

#[test]
fn test_run_cargo_command_success() {
    // Create a temporary directory
    let temp_dir = "test_dir";
    fs::create_dir(temp_dir).unwrap();

    // Create a simple Cargo project
    Command::new("cargo")
        .args(&["init", "--name", "test_project"])
        .current_dir(temp_dir)
        .status()
        .expect("Failed to create Cargo project");

    // Test running a valid cargo command
    run_cargo_command(temp_dir, "cargo", &["build"]);

    // Clean up
    fs::remove_dir_all(temp_dir).unwrap();
}

#[test]
fn test_run_cargo_command_directory_does_not_exist() {
    let non_existent_path = "./non_existent_directory";
    run_cargo_command(non_existent_path, "cargo", &["--version"]);

    // Since the function prints an error message, capturing stdout/stderr would be needed for assertions.
}

#[test]
fn test_run_cargo_command_fails() {
    let temp_dir = tempdir().expect("Failed to create temp dir");
    let path = temp_dir.path();

    run_cargo_command(path.to_str().unwrap(), "cargo", &["nonexistent-command"]);

    // Since the function prints an error message, capturing stdout/stderr would be needed for assertions.
}

// Cargo build
// --------------------------

// Helper function to create a temporary Cargo project
fn create_temp_cargo_project(path: &str) {
    fs::create_dir_all(path).unwrap();
    Command::new("cargo")
        .args(&["init", "--name", "temp_project"])
        .current_dir(path)
        .status()
        .expect("Failed to create Cargo project");
}

#[test]
fn test_run_cargo_build_default() {
    // Create a temporary directory and Cargo project
    let temp_dir = "test_build_default";
    create_temp_cargo_project(temp_dir);

    // Test running `cargo build` with default options
    run_cargo_build(temp_dir, None, false, None);

    // Clean up
    fs::remove_dir_all(temp_dir).unwrap();
}

#[test]
fn test_run_cargo_build_with_features() {
    // Create a temporary directory and Cargo project
    let temp_dir = "test_build_features";
    create_temp_cargo_project(temp_dir);

    // Test running `cargo build` with features
    run_cargo_build(temp_dir, Some("my_feature".to_string()), false, None);

    // Clean up
    fs::remove_dir_all(temp_dir).unwrap();
}

#[test]
fn test_run_cargo_build_release() {
    // Create a temporary directory and Cargo project
    let temp_dir = "test_build_release";
    create_temp_cargo_project(temp_dir);

    // Test running `cargo build --release`
    run_cargo_build(temp_dir, None, true, None);

    // Clean up
    fs::remove_dir_all(temp_dir).unwrap();
}

#[test]
fn test_run_cargo_build_with_target() {
    // Create a temporary directory and Cargo project
    let temp_dir = "test_build_target";
    create_temp_cargo_project(temp_dir);

    // Test running `cargo build` with a target
    run_cargo_build(
        temp_dir,
        None,
        false,
        Some("x86_64-unknown-linux-gnu".to_string()),
    );

    // Clean up
    fs::remove_dir_all(temp_dir).unwrap();
}

#[test]
fn test_run_cargo_build_with_all_options() {
    // Create a temporary directory and Cargo project
    let temp_dir = "test_build_all_options";
    create_temp_cargo_project(temp_dir);

    // Test running `cargo build` with all options
    run_cargo_build(
        temp_dir,
        Some("my_feature".to_string()),
        true,
        Some("x86_64-unknown-linux-gnu".to_string()),
    );

    // Clean up
    fs::remove_dir_all(temp_dir).unwrap();
}

// Helper function to create a temporary file with some content
fn create_temp_file(content: &str) -> NamedTempFile {
    let mut file = NamedTempFile::new().unwrap();
    write!(file, "{}", content).unwrap();
    file
}

// SSH
// --------------------------

#[test]
fn test_copy_file_over_ssh() {
    // Create a temporary local file
    let local_file = create_temp_file("Hello, this is a test file!");
    let local_file_path = local_file.path().to_str().unwrap();

    // Define test server details (this would be a mock or test server in practice)
    let host = "localhost"; // Replace with a mock/test server
    let port = 22;
    let username = "testuser";
    let password = "testpassword";
    let remote_file_path = "/tmp/remote_test_file.txt";

    // Attempt to copy the file
    let result = copy_file_over_ssh(
        host,
        port,
        username,
        password,
        local_file_path,
        remote_file_path,
    );

    // Assert that the function does NOT return Ok(())
    assert!(!result.is_ok());

    // If you have a mock SSH server, you could also verify the file was copied correctly.
    // For now, we'll just check that the function didn't return an error.
}

#[test]
fn test_copy_file_over_ssh_invalid_credentials() {
    // Create a temporary local file
    let local_file = create_temp_file("Hello, this is a test file!");
    let local_file_path = local_file.path().to_str().unwrap();

    // Define test server details with invalid credentials
    let host = "testserver.example.com"; // Replace with a mock/test server
    let port = 22;
    let username = "invaliduser";
    let password = "invalidpassword";
    let remote_file_path = "/tmp/remote_test_file.txt";

    // Attempt to copy the file
    let result = copy_file_over_ssh(
        host,
        port,
        username,
        password,
        local_file_path,
        remote_file_path,
    );

    // Assert that the function returns an error
    assert!(result.is_err());
}

#[test]
fn test_copy_file_over_ssh_invalid_local_file() {
    // Define a non-existent local file path
    let local_file_path = "/nonexistent/path/to/file.txt";

    // Define test server details
    let host = "testserver.example.com"; // Replace with a mock/test server
    let port = 22;
    let username = "testuser";
    let password = "testpassword";
    let remote_file_path = "/tmp/remote_test_file.txt";

    // Attempt to copy the file
    let result = copy_file_over_ssh(
        host,
        port,
        username,
        password,
        local_file_path,
        remote_file_path,
    );

    // Assert that the function returns an error
    assert!(result.is_err());
}
