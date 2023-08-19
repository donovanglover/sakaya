use clap::Parser;
use cli::Cli;
use sakaya::start_client;
use sakaya::start_server;
use std::net::IpAddr;
use std::net::SocketAddrV4;
use local_ip_address::local_ip;

mod cli;

/// The main function is in charge of either starting a `sakaya-server` or
/// starting a `sakaya-client` that connects to a `sakaya-server`.
///
/// It does this by checking if the --server flag was passed.
fn main() {
    #[rustfmt::skip]
    let Cli { address, server, .. } = Cli::parse();

    if let Ok(IpAddr::V4(ip)) = local_ip() {
        let running_ip = SocketAddrV4::new(ip, 39493);

        if server {
            start_server(running_ip);
        } else {
            start_client(address, "test");
        }
    }
}
