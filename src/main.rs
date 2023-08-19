extern crate pnet;

use clap::Parser;
use cli::Cli;
use sakaya::start_client;
use sakaya::start_server;
use local_ip_address::local_ip;
use pnet::datalink;

mod cli;

/// The main function is in charge of either starting a `sakaya-server` or
/// starting a `sakaya-client` that connects to a `sakaya-server`.
///
/// It does this by checking if the --server flag was passed.
fn main() {
    #[rustfmt::skip]
    let Cli { address, server, .. } = Cli::parse();

    let my_local_ip = local_ip();

    if let Ok(my_local_ip) = my_local_ip {
        println!("This is my local IP address: {:?}", my_local_ip);
    } else {
        println!("Error getting local IP: {:?}", my_local_ip);
    }

    for iface in datalink::interfaces() {
        for ip in iface.ips {
            println!("{:?}", ip)
        }
    }

    if server {
        start_server(address);
    } else {
        start_client(address, "test");
    }
}
