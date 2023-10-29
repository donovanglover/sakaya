use clap::Parser;
use cli::Cli;
use cli::Commands;
use local_ip_address::local_ip;
use sakaya::is_container;
use sakaya::notify;
use std::net::IpAddr;
use std::net::SocketAddrV4;

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
    let Cli { address, command, file, directory } = Cli::parse();

    let server = command == Some(Commands::Server {});

    if let Ok(IpAddr::V4(ip)) = local_ip() {
        let running_ip = SocketAddrV4::new(ip, 39493);

        if server || is_container() {
            notify(&format!("Starting server on {running_ip}..."), None);
            server::start(running_ip);
        } else if let Some(file) = file {
            client::exec(address, &file, directory.to_str().unwrap());
        } else {
            notify("sakaya was called but no file was given.", None);
        }
    }
}
