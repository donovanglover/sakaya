use clap::Parser;
use std::path::PathBuf;
use std::net::IpAddr;

#[derive(Parser)]
#[command(version)]
struct Args {
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

fn main() {
    let args = Args::parse();

    println!("{}", args.host_address)
}
