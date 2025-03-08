use crate::local::{run_cargo_command, run_local_command};
use crate::remote::{journal_over_ssh, run_over_ssh};
use crate::{copy_file_over_ssh, create_servicefile};

pub fn test_mode(code_path: &str) {
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

pub fn sim_mode(code_path: &str) {
    println!("Running local Simulation...");
    match run_cargo_command(
        code_path,
        "cargo",
        &["build", "--features", "sim", "--release"],
    ) {
        Ok(msg) => {
            // TODO: maybe test these as well?
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

pub fn build_mode(code_path: &str, username: &str) {
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
            "--target=aarch64-unknown-linux-gnu",
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
    local_binary_path: &str,
    remote_binary_path: &str,
) {
    println!("Copying System Service files to Bot...");
    let local_svcfile = "homebot.service";
    let remote_tmpsvcfile = format!("/home/{}/{}", username, local_svcfile);
    copy_file_over_ssh(
        host,
        port,
        username,
        password,
        ssh_key_path,
        &local_svcfile,
        &remote_tmpsvcfile,
    )
    .expect("ERROR Copying system service file to Bot!");

    println!("Moving file to System Service folder...");
    let comm_cp_systemd = format!(
        "sudo cp {} /etc/systemd/system/{}",
        remote_tmpsvcfile, local_svcfile
    );
    let run_comm_cp_systemd = run_over_ssh(
        host,
        port,
        username,
        password,
        ssh_key_path,
        &comm_cp_systemd,
    );
    println!("Result: {:#?}", run_comm_cp_systemd);

    println!("Configuring System Service...");
    let comm_systemd = format!(
        "sudo systemctl daemon-reload && sudo systemctl enable {}",
        local_svcfile
    );
    let run_comm_systemd =
        run_over_ssh(host, port, username, password, ssh_key_path, &comm_systemd);
    println!("Result: {:#?}", run_comm_systemd);

    println!("Cleaning up previous binary...");
    let comm_rm = format!("rm {}", remote_binary_path);
    let run_comm_rm = run_over_ssh(host, port, username, password, ssh_key_path, &comm_rm);
    println!("Result: {:#?}", run_comm_rm);

    println!("Copying binary to Bot...");
    match copy_file_over_ssh(
        host,
        port,
        username,
        password,
        ssh_key_path,
        &local_binary_path,
        &remote_binary_path,
    ) {
        Ok(msg) => {
            println!("Result: {:#?}", msg);
            println!("Making Binary Executable...");
            let comm_chmod = format!("chmod +x {}", remote_binary_path);

            match run_over_ssh(host, port, username, password, ssh_key_path, &comm_chmod) {
                Ok(msg) => {
                    println!("Result: {:#?}", msg);
                    let comm_run = format!("sudo systemctl start {}", local_svcfile);

                    match run_over_ssh(host, port, username, password, ssh_key_path, &comm_run) {
                        Ok(msg) => {
                            println!("Result: {:#?}", msg);
                            let svcname = "homebot";
                            match journal_over_ssh(
                                host,
                                port,
                                username,
                                password,
                                ssh_key_path,
                                &svcname,
                            ) {
                                Ok(msg) => {
                                    println!("Result: {:#?}", msg);
                                }
                                Err(e) => {
                                    println!("ERROR Tailing the logfile: {:#?}", e);
                                }
                            }
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
