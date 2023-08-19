use clap::Parser;
use std::net::SocketAddrV4;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about)]
pub struct Cli {
    /// Path to the executable to run.
    pub file: Option<PathBuf>,

    /// Address
    #[arg(short, long, default_value = "192.168.100.49:39493")]
    pub address: SocketAddrV4,

    /// Start a server instead of a client
    #[arg(short, long)]
    pub server: bool,
}
