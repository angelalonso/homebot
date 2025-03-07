use crate::{copy_file_over_ssh};
use crate::remote::run_over_ssh;

pub fn deploy(
    host: &str,
    port: u16,
    username: &str,
    password: Option<&str>,
    ssh_key_path: Option<&str>,
    local_file_path: &str,
    remote_file_path: &str,
) -> Result<String, String> {
    println!("Cleaning up previous binary...");
    let mut comm_rm = "rm ".to_owned();
    comm_rm.push_str(&remote_file_path);
    let run_comm_rm = run_over_ssh(host, port, username, password, ssh_key_path, &comm_rm);
    println!("Result: {:#?}", run_comm_rm);

    println!("Copying binary to Bot...");
    match copy_file_over_ssh(
        host,
        port,
        username,
        password,
        ssh_key_path,
        &local_file_path,
        &remote_file_path,
    ) {
        Ok(msg) => {
            println!("Result: {:#?}", msg);
            println!("Making Binary Executable...");
            let mut comm_chmod = "chmod +x ".to_owned();
            comm_chmod.push_str(&remote_file_path);

            match run_over_ssh(host, port, username, password, ssh_key_path, &comm_chmod) {
                Ok(msg) => {
                    println!("Result: {:#?}", msg);
                    let mut comm_run = "".to_owned();
                    comm_run.push_str(&remote_file_path);

                    match run_over_ssh(host, port, username, password, ssh_key_path, &comm_run) {
                        Ok(msg) => {
                            println!("Result: {:#?}", msg);
                            Ok("OK".to_string())
                        }
                        Err(e) => {
                            println!("ERROR Running the binary: {:#?}", e);
                            Err("ERROR".to_string())
                        }
                    }
                }
                Err(e) => {
                    println!("ERROR Chmoding the binary: {:#?}", e);
                    Err("ERROR".to_string())
                }
            }
        }
        Err(e) => {
            println!("ERROR SCP'ing to the host: {:#?}", e);
            Err("ERROR".to_string())
        }
    }
}

