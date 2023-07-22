use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::net::IpAddr;
use std::process::Command;

const STATE_DIRECTORY: &str = "/var/lib/nixos-containers";
const CONFIG_DIRECTORY: &str = "/etc/nixos-containers";

#[derive(Parser)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// List all containers.
    List {},

    /// Start an existing container.
    Start {
        container_name: String
    },

    /// Stop an existing container.
    Stop {
        container_name: String
    },

    /// Terminate a container.
    Terminate {
        container_name: String
    },

    /// Get the status of a container.
    Status {
        container_name: String
    },

    /// Update a container.
    Update {
        container_name: String
    },

    /// Login to a container.
    Login {
        container_name: String
    },

    /// Run a command from a container.
    Run {
        container_name: String
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::List {}) => {
            let _ = Command::new("nixos-container")
                .arg("list")
                .spawn();
        }

        Some(Commands::Start { container_name }) => {
            let _ = Command::new("machinectl")
                .args(["start", container_name])
                .spawn();
        }

        Some(Commands::Stop { container_name }) => {
            let _ = Command::new("machinectl")
                .args(["stop", container_name])
                .spawn();
        }

        Some(Commands::Terminate { container_name }) => {
            let _ = Command::new("machinectl")
                .args(["terminate", container_name])
                .spawn();
        }

        Some(Commands::Status { container_name }) => {
            let _ = Command::new("machinectl")
                .args(["status", container_name])
                .spawn();
        }

        Some(Commands::Update { .. }) => {}

        Some(Commands::Login { container_name }) => {
            let _ = Command::new("machinectl")
                .args(["login", "--", container_name])
                .spawn();
        }


        Some(Commands::Run { .. }) => {}

        None => {}
    }
}
