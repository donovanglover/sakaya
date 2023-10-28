use clap::builder::styling::{AnsiColor, Effects, Styles};
use clap::Parser;
use std::net::SocketAddrV4;
use std::path::PathBuf;

fn styles() -> Styles {
    Styles::styled()
        .header(AnsiColor::Red.on_default() | Effects::BOLD)
        .usage(AnsiColor::Red.on_default() | Effects::BOLD)
        .literal(AnsiColor::Blue.on_default() | Effects::BOLD)
        .placeholder(AnsiColor::Green.on_default())
}

#[derive(Parser)]
#[command(author, version, about, styles = styles())]
pub struct Cli {
    /// Path to the executable to run.
    pub file: Option<PathBuf>,

    /// Address of the server to request (client only)
    #[arg(short, long, default_value = "192.168.100.49:39493")]
    pub address: SocketAddrV4,

    /// Host directory mounted to /mnt inside the container (client only)
    #[arg(short, long, default_value = "/home/user/containers/wine")]
    pub directory: PathBuf,

    /// Start a server instead of a client
    #[arg(short, long)]
    pub server: bool,
}
