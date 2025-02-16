use clap::{Parser, Subcommand};
use homebotctl::cfg::Config;
use homebotctl::{copy_file_over_ssh, run_cargo_command, run_local_command};

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
    Deploy {
        //        path: String,
        //        host: String,
        //        port: u16,
        //        username: String,
        //        password: String,
        //        local_file_path: String,
        //        remote_file_path: String,
    },
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
        Commands::Test {} => run_cargo_command(
            &cfg.code_path,
            "cargo",
            &["test", "--features", "test", "--", "--nocapture"],
        ),
        Commands::Sim {} => {
            run_cargo_command(
                &cfg.code_path,
                "cargo",
                &["build", "--features", "sim", "--release"],
            );
            run_local_command("mkdir -p ../simulation/controllers/rust_controller/");
            run_local_command(
                "cp ../target/release/homebot ../simulation/controllers/rust_controller/rust_controller",
            );
            run_local_command("cp ../cfg.yaml ../simulation/controllers/rust_controller/");
            run_local_command("webots ../simulation/worlds/homebot_simulation_world.wbt");
        }
        Commands::Build {} => {
            run_cargo_command(
                &cfg.code_path,
                "cargo",
                &["test", "--features", "test", "--", "--nocapture"],
            );
            run_cargo_command(
                &cfg.code_path,
                "cargo",
                &[
                    "build",
                    "--features",
                    "live",
                    "--release",
                    "--target=aarch64-unknown-linux-gnu",
                    //"--target=aarch64-unknown-linux-musl",
                ],
            );
        }
        Commands::Deploy {} => {
            let local_file_path = "../target/aarch64-unknown-linux-gnu/release/homebot";
            let remote_file_path = "/home/aafmin/homebot";
            copy_file_over_ssh(
                &cfg.host,
                cfg.port,
                &cfg.username,
                Some(&cfg.password),
                Some(&cfg.ssh_key_path),
                &local_file_path,
                &remote_file_path,
            )
            .expect("ERROR SSH'ing into the host")
        } //        Commands::Stop {
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
