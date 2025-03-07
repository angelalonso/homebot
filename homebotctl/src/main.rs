use clap::{Parser, Subcommand};
use homebotctl::cfg::Config;
use homebotctl::modes::{test_mode, sim_mode, build_mode, deploy_mode};
use homebotctl::{get_ips_open, is_bot_online};

#[derive(Parser)]
#[command(name = "cargo-runner")]
#[command(about = "A tool to run cargo commands in a specified directory", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Test {},
    Sim {},
    Build {},
    Deploy {},
    //    Stop {
    //        host: String,
    //        port: u16,
    //        username: String,
    //        password: String,
    //        local_file_path: String,
    //        remote_file_path: String,
    //    },
}

fn main() {
    let cfgfile_path = "ctlcfg.yml";
    let cfg = Config::from_file(&cfgfile_path).unwrap();
    let cli = Cli::parse();

    match cli.command {
        Commands::Test {} => test_mode(
            &cfg.code_path,
        ),
        Commands::Sim {} => sim_mode(
            &cfg.code_path,
        ),
        Commands::Build {} => build_mode(
            &cfg.code_path,
            &cfg.username,
        ),
        Commands::Deploy {} => match is_bot_online(&cfg.host, cfg.port) { // This could be moved to
                                                                          // modes.rs, but it's ok
            true => {
                let local_file_path = "../target/aarch64-unknown-linux-gnu/release/homebot";
                let mut remote_file_path: String = "/home/".to_owned();
                remote_file_path.push_str(&cfg.username);
                remote_file_path.push_str("/homebot");
                let _ = deploy_mode(
                    &cfg.host,
                    cfg.port,
                    &cfg.username,
                    Some(&cfg.password),
                    Some(&cfg.ssh_key_path),
                    local_file_path,
                    &remote_file_path,
                );
            }
            false => {
                println!("WARNING - Robot is not online or changed IP, let me check if it's on a different one...");
                get_ips_open(&cfg.lan_base, cfg.lan_mask, cfg.port);
            }
        }, //        Commands::Stop {
           //            host,
           //            port,
           //            username,
           //            password,
           //            local_file_path,
           //            remote_file_path,
           //        } => copy_file_over_ssh(
           //            &host,
           //            port,
           //            &username,
           //            &password,
           //            &local_file_path,
           //            &remote_file_path,
           //        )
           //        .expect("ERROR SSH'ing into the host"),
    }
}
