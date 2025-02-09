use clap::{Parser, Subcommand};
use homebotctl::{copy_file_over_ssh, run_cargo_build, run_cargo_command};

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
        path: String,
    },
    Sim {
        path: String,
    },
    Build {
        path: String,
        #[arg(long)]
        features: Option<String>,
        #[arg(long)]
        release: bool,
        #[arg(long)]
        target: Option<String>,
    },
    Deploy {
        path: String,
        host: String,
        port: u16,
        username: String,
        password: String,
        local_file_path: String,
        remote_file_path: String,
    },
    Stop {
        host: String,
        port: u16,
        username: String,
        password: String,
        local_file_path: String,
        remote_file_path: String,
    },
}

fn main() {
    let cli = Cli::parse();

    // TODO: use the proper functions (run_local_command, run_over_ssh)
    match cli.command {
        Commands::Test { path } => run_cargo_command(&path, "cargo", &["test"]),
        Commands::Sim { path } => run_cargo_command(&path, "cargo", &["test"]),
        Commands::Build {
            path,
            features,
            release,
            target,
        } => run_cargo_build(&path, features, release, target),
        Commands::Deploy {
            path,
            host,
            port,
            username,
            password,
            local_file_path,
            remote_file_path,
        } => copy_file_over_ssh(
            &host,
            port,
            &username,
            &password,
            &local_file_path,
            &remote_file_path,
        )
        .expect("ERROR SSH'ing into the host"),
        Commands::Stop {
            host,
            port,
            username,
            password,
            local_file_path,
            remote_file_path,
        } => copy_file_over_ssh(
            &host,
            port,
            &username,
            &password,
            &local_file_path,
            &remote_file_path,
        )
        .expect("ERROR SSH'ing into the host"),
    }
}
