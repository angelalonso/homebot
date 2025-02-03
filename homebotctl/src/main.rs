use clap::{Parser, Subcommand};
use std::path::Path;
use std::process::Command;

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
        Commands::Run { path } => run_cargo_command(&path, "cargo", &["run"]),
    }
}

fn run_cargo_build(path: &str, features: Option<String>, release: bool, target: Option<String>) {
    let mut args = vec!["build".to_string()];

    // Add --features if specified
    if let Some(features) = features {
        args.push("--features".to_string());
        args.push(features);
    }

    // Add --release if specified
    if release {
        args.push("--release".to_string());
    }

    // Add --target if specified
    if let Some(target) = target {
        args.push("--target".to_string());
        args.push(target);
    }

    // Convert Vec<String> to Vec<&str> for Command::args
    let args_ref: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
    run_cargo_command(path, "cargo", &args_ref);
}

fn run_cargo_command(path: &str, command: &str, args: &[&str]) {
    // Check if the directory exists
    if !Path::new(path).exists() {
        eprintln!("Error: Directory '{}' does not exist.", path);
        return;
    }

    // Run the cargo command
    let mut cmd = Command::new(command);
    cmd.args(args).current_dir(path);

    println!("Running: {} {} in {}", command, args.join(" "), path);

    let status = cmd
        .status()
        .expect(&format!("Failed to execute '{}'", command));

    // Check if the command succeeded
    if !status.success() {
        eprintln!(
            "'{} {}' failed with exit code: {:?}",
            command,
            args.join(" "),
            status.code()
        );
    }
}
