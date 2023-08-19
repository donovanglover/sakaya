use clap::Parser;
use cli::Cli;
use sakaya::start_client;
use sakaya::start_server;

mod cli;

/// The main function is in charge of either starting a `sakaya-server` or
/// starting a `sakaya-client` that connects to a `sakaya-server`.
///
/// It does this by
fn main() {
    #[rustfmt::skip]
    let Cli { address, server, .. } = Cli::parse();

    if server {
        start_server(address);
    } else {
        start_client(address, "test");
    }
}
