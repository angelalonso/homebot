use crate::{copy_file_over_ssh, create_servicefile};
use crate::remote::run_over_ssh;
use crate::local::{run_cargo_command, run_local_command};

pub fn test_mode(
    code_path: &str,
) {
    println!("Testing local code...");
    match run_cargo_command(
            code_path,
            "cargo",
            &["test", "--features", "test", "--", "--nocapture"],
    ) {
        Ok(msg) => {
            println!("Result: {:#?}", msg);
        }
        Err(e) => {
            println!("ERROR Testing local code: {:#?}", e);
        }
    }
}

pub fn sim_mode(
    code_path: &str
) {
    println!("Running local Simulation...");
    match run_cargo_command(
        code_path,
        "cargo",
        &["build", "--features", "sim", "--release"],
    ) {
        Ok(msg) => { // TODO: maybe test these as well?
            println!("Result: {:#?}", msg);
            run_local_command("mkdir -p ../simulation/controllers/rust_controller/");
            run_local_command(
                "cp ../target/release/homebot ../simulation/controllers/rust_controller/rust_controller",
            );
            run_local_command("cp ../cfg.yaml ../simulation/controllers/rust_controller/");
            run_local_command("webots ../simulation/worlds/homebot_simulation_world.wbt");
        }
        Err(e) => {
            println!("ERROR Simulating locally: {:#?}", e);
        }
    }
}

pub fn build_mode(
    code_path: &str,
    username: &str,
) {
    println!("Before Building code:");
    test_mode(code_path);
    println!("Building code...");
    match run_cargo_command(
                code_path,
                "cargo",
                &[
                    "build",
                    "--features",
                    "live",
                    "--release",
                    //"--target=aarch64-unknown-linux-gnu",
                    //"--target=aarch64-unknown-linux-musl",
                ],
    ) {
        Ok(msg) => {
            println!("Result: {:#?}", msg);
        }
        Err(e) => {
            println!("ERROR Building code: {:#?}", e);
        }
    };
    create_servicefile(username);
}

pub fn deploy_mode(
    host: &str,
    port: u16,
    username: &str,
    password: Option<&str>,
    ssh_key_path: Option<&str>,
    local_file_path: &str,
    remote_file_path: &str,
) {
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
                        }
                        Err(e) => {
                            println!("ERROR Running the binary: {:#?}", e);
                        }
                    }
                }
                Err(e) => {
                    println!("ERROR Chmoding the binary: {:#?}", e);
                }
            }
        }
        Err(e) => {
            println!("ERROR SCP'ing to the host: {:#?}", e);
        }
    }
}

