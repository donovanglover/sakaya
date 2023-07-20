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

    /// Create a new container.
    Create {
        container_name: String,

        #[arg(long)]
        nixos_path: PathBuf,

        #[arg(long)]
        system_path: PathBuf,

        #[arg(long)]
        config: String,

        #[arg(long)]
        config_file: PathBuf,

        // TODO: flakeref
        #[arg(long)]
        flake: String,

        #[arg(long)]
        ensure_unique_name: bool,

        #[arg(long)]
        auto_start: bool,

        // TODO: iface
        #[arg(long)]
        bridge: String,

        // TODO: port
        #[arg(long)]
        port: String,

        #[arg(long)]
        host_address: IpAddr,

        #[arg(long)]
        local_address: IpAddr,
    },

    /// Destroy an existing container.
    Destroy {
        container_name: String
    },

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

    /// Login as root to a container.
    RootLogin {
        container_name: String
    },

    /// Run a command from a container.
    Run {
        container_name: String
    },

    /// Show the IP address of a container.
    ShowIp {
        container_name: String
    },

    /// Show the host key of a container.
    ShowHostKey {
        container_name: String
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::List { .. }) => {}

        Some(Commands::Create { auto_start, .. }) => {
            if *auto_start {
                println!("auto_start");
            }
        }

        Some(Commands::Destroy { .. }) => {}

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

        Some(Commands::Terminate { .. }) => {}

        Some(Commands::Status { .. }) => {}

        Some(Commands::Update { .. }) => {}

        Some(Commands::Login { container_name }) => {
            let _ = Command::new("machinectl")
                .args(["login", "--", container_name])
                .spawn();
        }

        Some(Commands::RootLogin { .. }) => {}

        Some(Commands::Run { .. }) => {}

        Some(Commands::ShowIp { .. }) => {}

        Some(Commands::ShowHostKey { .. }) => {}

        None => {}
    }
}
