use homebotctl::remote::*;
use homebotctl::local::*;
use homebotctl::*;
use std::fs;
use std::io::Write;
use tempfile::NamedTempFile;

// Helper function to create a temporary file with some content
fn create_temp_file(content: &str) -> NamedTempFile {
    let mut file = NamedTempFile::new().unwrap();
    write!(file, "{}", content).unwrap();
    file
}

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
#[cfg(feature = "botonline")]
#[test]
fn test_run_ssh_command() {
    let cfgfile_path = "ctlcfg.yml";
    let cfg = Config::from_file(&cfgfile_path).unwrap();
    let command = "echo Hello, SSH!";

    // Run the SSH command
    let result = run_over_ssh(
        &cfg.host,
        cfg.port,
        &cfg.username,
        Some(&cfg.password),
        Some(&cfg.ssh_key_path),
        command,
    );

    // Check if the command was successful
    match result {
        Ok(output) => {
            assert_eq!(output, "Hello, SSH!\n");
        }
        Err(e) => {
            assert!(e.contains("failed to lookup address information"));
            panic!("SSH command SHOULD NOT have failed: {}", e);
        }
    }
}

#[test]
fn test_run_ssh_command_fail() {
    // Replace these with your SSH server details
    let host = "your.ssh.server.com";
    let port = 22;
    let username = "your_username";
    let password = Some("your_password");
    let ssh_key_path = None;
    let command = "echo Hello, SSH!";

    // Run the SSH command
    let result = run_over_ssh(host, port, username, password, ssh_key_path, command);
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

// Copy over SSH
// -------------
#[cfg(feature = "botonline")]
#[test]
fn test_copy_file_over_ssh() {
    let cfgfile_path = "ctlcfg.yml";
    let cfg = Config::from_file(&cfgfile_path).unwrap();
    let local_file_path = "cargotest_local_file.tmp";
    let remote_file_path = "/home/".to_owned() + &cfg.username + "/cargotest_remote_file.tmp";

    // Create a test file locally
    let test_content = "This is a test file from running cargo tests.";
    fs::write(local_file_path, test_content).expect("Failed to create local test file");

    // Call the function to copy the file over SSH
    let result = copy_file_over_ssh(
        &cfg.host,
        cfg.port,
        &cfg.username,
        Some(&cfg.password),
        Some(&cfg.ssh_key_path),
        local_file_path,
        &remote_file_path,
    );

    // Clean up: Delete the local test file
    fs::remove_file(local_file_path).expect("Failed to delete local test file");

    // Assert that the function succeeded
    assert!(result.is_ok());

    // Optional: Verify the file was copied correctly by reading it back (if you have SSH access)
    // This step is optional and depends on your test environment.
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
    let password = Some("password"); // Set to `None` if using SSH key
    let ssh_key_path = None; // Set to `None` if using password
    let remote_file_path = "/tmp/remote_test_file.txt";

    // Attempt to copy the file
    let result = copy_file_over_ssh(
        host,
        port,
        username,
        password,
        ssh_key_path,
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
    let password = None; // Set to `None` if using SSH key
    let ssh_key_path = Some("/path/to/id_rsa"); // Set to `None` if using password
    let remote_file_path = "/tmp/remote_test_file.txt";

    // Attempt to copy the file
    let result = copy_file_over_ssh(
        host,
        port,
        username,
        password,
        ssh_key_path,
        local_file_path,
        remote_file_path,
    );

    // Assert that the function returns an error
    assert!(result.is_err());
}
