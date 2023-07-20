use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::net::IpAddr;

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
    Destroy {},

    /// Start an existing container.
    Start {},

    /// Stop an existing container.
    Stop {},

    /// Terminate a container.
    Terminate {},

    /// Get the status of a container.
    Status {},

    /// Update a container.
    Update {},

    /// Login to a container.
    Login {},

    /// Login as root to a container.
    RootLogin {},

    /// Run a command from a container.
    Run {},

    /// Show the IP address of a container.
    ShowIp {},

    /// Show the host key of a container.
    ShowHostKey {},
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

        Some(Commands::Start { .. }) => {}

        Some(Commands::Stop { .. }) => {}

        Some(Commands::Terminate { .. }) => {}

        Some(Commands::Status { .. }) => {}

        Some(Commands::Update { .. }) => {}

        Some(Commands::Login { .. }) => {}

        Some(Commands::RootLogin { .. }) => {}

        Some(Commands::Run { .. }) => {}

        Some(Commands::ShowIp { .. }) => {}

        Some(Commands::ShowHostKey { .. }) => {}

        None => {}
    }
}
