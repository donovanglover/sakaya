use clap::Parser;
use cli::Cli;
use sakaya::*;

mod cli;

fn main() {
    let cli = Cli::parse();

    if is_container() {
        server("127.0.0.1:7878");
        // server("192.168.100.49:39493");
    } else {
        client(&cli.executable)
    }
}
