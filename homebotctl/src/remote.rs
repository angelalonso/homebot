use crate::{
    copy_file_over_ssh, get_ips_open, is_bot_online, run_cargo_command, run_local_command,
    run_over_ssh,
};

pub fn deploy(
    host: &str,
    port: u16,
    username: &str,
    password: Option<&str>,
    ssh_key_path: Option<&str>,
    local_file_path: &str,
    remote_file_path: &str,
) -> Result<String, String> {
    let run = |cmd: &str| run_over_ssh(host, port, username, password, ssh_key_path, cmd);
    let copy = || {
        copy_file_over_ssh(
            host,
            port,
            username,
            password,
            ssh_key_path,
            &local_file_path,
            &remote_file_path,
        )
    };

    println!("Cleaning up previous binary...");
    let mut comm_rm = "rm ".to_owned();
    comm_rm.push_str(&remote_file_path);
    let run_comm_rm = run_over_ssh(host, port, username, password, ssh_key_path, &comm_rm);
    println!("Result: {:#?}", run_comm_rm);

    println!("Copying binary to Bot...");
    if let Ok(msg) = copy() {
        println!("Result: {:#?}", msg);
        println!("Running a test...");
        let run_comm_chmod = run(&format!("chmod +x {}", remote_file_path));
        println!("{:#?}", run_comm_chmod);
    } else {
        println!("ERROR SCP'ing to the host");
    }

    Ok("Deployment completed".to_string())
}
