use clap::{Parser, Subcommand};
use homebotctl::{copy_file_over_ssh, run_cargo_build, run_cargo_command};
use homebotctl::cfg::Config;

#[derive(Parser)]
#[command(name = "cargo-runner")]
#[command(about = "A tool to run cargo commands in a specified directory", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Test {
//        path: String,
    },
// TODO: do this
//    Sim {
//        path: String,
//    },
    Build {
//        path: String,
//        #[arg(long)]
//        features: Option<String>,
//        #[arg(long)]
//        release: bool,
//        #[arg(long)]
//        target: Option<String>,
    },
//    Deploy {
//        path: String,
//        host: String,
//        port: u16,
//        username: String,
//        password: String,
//        local_file_path: String,
//        remote_file_path: String,
//    },
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

    // TODO: use the proper functions (run_local_command, run_over_ssh)
    match cli.command {
        Commands::Test {} => run_cargo_command(&cfg.code_path, "cargo", &["test", "--features", "test", "--", "--nocapture"]),
//        Commands::Sim { path } => run_cargo_command(&code_path, "cargo", &["test"]),
        Commands::Build {} => run_cargo_command(&cfg.code_path, "cargo", &["build", "--features", "live", "--release", "--target=armv7-unknown-linux-gnueabihf"]),
//        Commands::Build {
//            path,
//            features,
//            release,
//            target,
//        } => run_cargo_build(&path, features, release, target),
//        Commands::Deploy {
//            cfg.host,
//            cfg.port,
//            cfg.username,
//            cfg.password,
//            local_file_path,
//            remote_file_path,
//        } => copy_file_over_ssh(
//            cfg.host,
//            cfg.port,
//            cfg.username,
//            cfg.password,
//            &local_file_path,
//            &remote_file_path,
//        )
//        .expect("ERROR SSH'ing into the host"),
//        Commands::Stop {
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
