use clap::builder::styling::{AnsiColor, Effects, Styles};
use clap::{Parser, Subcommand};
use std::net::SocketAddrV4;
use std::path::PathBuf;

use crate::consts::{
    DEFAULT_ADDRESS, DEFAULT_DIRECTORY, DEFAULT_LOCALE, DEFAULT_PORT, DEFAULT_TIMEZONE, DEFAULT_WINE32_PREFIX, DEFAULT_WINE64_PREFIX
};

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

    /// Address of the server to request
    #[arg(short, long, default_value = DEFAULT_ADDRESS)]
    pub address: SocketAddrV4,

    /// Host directory mounted to /mnt inside the container
    #[arg(short, long, default_value = DEFAULT_DIRECTORY)]
    pub directory: PathBuf,

    /// $WINEPREFIX for 32-bit applications (i386)
    #[arg(short, long, default_value = DEFAULT_WINE32_PREFIX)]
    pub wine32: String,

    /// $WINEPREFIX for 64-bit applications (amd64)
    #[arg(short = 'W', long, default_value = DEFAULT_WINE64_PREFIX)]
    pub wine64: String,

    /// Locale to run programs with
    #[arg(short, long, default_value = DEFAULT_LOCALE)]
    pub locale: String,

    /// Timezone to run programs with
    #[arg(short, long, default_value = DEFAULT_TIMEZONE)]
    pub timezone: String,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, PartialEq)]
pub enum Commands {
    /// Start a sakaya server instead of a client
    ///
    /// You shouldn't need to use this unless you want to start a sakaya server outside
    /// of a systemd-nspawn container.
    Server {
        /// Port number to use for the sakaya server
        #[arg(short, long, default_value_t = DEFAULT_PORT)]
        port: u16,
    },
}
