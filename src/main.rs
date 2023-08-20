use clap::Parser;
use cli::Cli;
use local_ip_address::local_ip;
use std::net::IpAddr;
use std::net::SocketAddrV4;

mod cli;
mod runner;
mod server;

/// The main function is in charge of either starting a `sakaya-server` or
/// starting a `sakaya-client` that connects to a `sakaya-server`.
///
/// It does this by checking if the --server flag was passed.
fn main() {
    #[rustfmt::skip]
    let Cli { address, server, file } = Cli::parse();

    if let Ok(IpAddr::V4(ip)) = local_ip() {
        let running_ip = SocketAddrV4::new(ip, 39493);

        if server {
            server::start(running_ip);
        } else {
            if let Some(file) = file {
                runner::exec(address, &file);
            } else {
                println!("Not a server file was given.");
            }
        }
    }
}
