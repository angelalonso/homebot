use homebotctl::{run_over_ssh};

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
            assert!(e.contains("failed to lookup address information: Name or service not known"));
        }
    }
}
//TODO: test positive response, maybe?
