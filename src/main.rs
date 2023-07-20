use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::net::IpAddr;

#[derive(Parser)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new nixos-container.
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
    }
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Create { auto_start, .. }) => {
            if *auto_start {
                println!("auto_start");
            }
        }
        None => {}
    }
}
