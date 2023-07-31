use clap::{Parser, Subcommand};
use std::process::Command;

#[derive(Parser)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Start sakaya in a new container or reboot an existing one.
    New {
        container_name: String
    },

    /// Show a list of processes running in a container.
    Ps {
        container_name: String
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::New { container_name }) => {
            let status = Command::new("nixos-container").args(["status", container_name]).output().expect("Failed to get nixos-container status");

            if String::from_utf8_lossy(&status.stdout) == "down\n" {
                let _ = Command::new("nixos-container").args(["start", container_name]).output().expect("Failed to run nixos-container start");
            } else {
                let _ = Command::new("machinectl").args(["reboot", container_name]).output().expect("Failed to run machinectl reboot");
            }

            let _ = Command::new("machinectl").args(["shell", "user@wine", "/usr/bin/env", "sakaya-server"]).output().expect("Failed to start a user shell with sakaya-server");
        }

        Some(Commands::Ps { container_name }) => {
            let _ = Command::new("machinectl").args(["status", container_name, "--no-pager"]).spawn().expect("Failed to get machinectl status");
        }

        None => {}
    }
}
