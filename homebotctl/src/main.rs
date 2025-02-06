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
    /// Run `cargo build` in the specified directory
    Build {
        /// Path to the directory containing the Cargo project
        path: String,
        /// Enable specific features (e.g., "live")
        #[arg(long)]
        features: Option<String>,
        /// Build in release mode
        #[arg(long)]
        release: bool,
        /// Target architecture (e.g., "armv7-unknown-linux-gnueabihf")
        #[arg(long)]
        target: Option<String>,
    },
    /// Run `cargo test` in the specified directory
    Test {
        /// Path to the directory containing the Cargo project
        path: String,
    },
    /// Run `cargo run` in the specified directory
    Deploy {
        host: String,
        port: u16,
        username: String,
        password: String,
        local_file_path: String,
        remote_file_path: String,
    },
    /// Run `cargo run` in the specified directory
    Run {
        /// Path to the directory containing the Cargo project
        path: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Build {
            path,
            features,
            release,
            target,
        } => run_cargo_build(&path, features, release, target),
        Commands::Test { path } => run_cargo_command(&path, "cargo", &["test"]),
        Commands::Deploy {
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
        Commands::Run { path } => run_cargo_command(&path, "cargo", &["run"]),
    }
}
