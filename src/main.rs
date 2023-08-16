use clap::Parser;
use cli::Cli;
use sakaya::*;

mod cli;

/// The main function is in charge of either starting a `sakaya-server` or
/// starting a `sakaya-client` that connects to a `sakaya-server`.
///
/// It does this by
fn main() {
    let Cli { address, server, .. } = Cli::parse();

    if server {
        start_server(address);
    } else {
        start_client(address, "test");
    }
}
