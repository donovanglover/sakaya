use clap::Parser;
use cli::Cli;
use cli::Commands;
use local_ip_address::local_ip;
use sakaya::is_container;
use sakaya::notify;
use std::net::SocketAddrV4;
use std::net::{IpAddr, Ipv4Addr};

mod cli;
mod client;
mod server;

/// The main function is in charge of either starting a `sakaya-server` or
/// starting a `sakaya-client` that connects to a `sakaya-server`.
///
/// It does this by checking if the --server flag was passed. It also defaults
/// to starting a `sakaya-server` if ran inside a systemd-nspawn container.
fn main() {
    #[rustfmt::skip]
    let Cli { address, command, file, directory, .. } = Cli::parse();

    if let Ok(IpAddr::V4(ip)) = local_ip() {
        match &command {
            Some(Commands::Server { port }) => start_server(ip, *port),

            None => {
                if is_container() {
                    start_server(ip, 39493)
                } else if let Some(file) = file {
                    if let Some(directory) = directory.to_str() {
                        client::exec(address, &file, directory);
                    } else {
                        notify("Invalid directory was given.", None)
                    }
                } else {
                    notify("sakaya was called but no file was given.", None);
                }
            }
        }
    }
}

fn start_server(ip: Ipv4Addr, port: u16) {
    let running_ip = SocketAddrV4::new(ip, port);
    notify(&format!("Starting server on {running_ip}..."), None);
    server::start(running_ip);
}
