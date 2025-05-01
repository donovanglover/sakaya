use clap::Parser;
use sakaya::cli::Cli;
use sakaya::cli::Commands;
use sakaya::util::is_container;
use sakaya::util::notify;
use sakaya::{client, server};
use std::net::Ipv4Addr;
use std::net::SocketAddrV4;
use std::path::Component;

/// The main function is in charge of either starting a `sakaya-server` or
/// starting a `sakaya-client` that connects to a `sakaya-server`.
///
/// It does this by checking if the `server` command was passed. It also defaults
/// to starting a `sakaya-server` if ran inside a systemd-nspawn container.
#[tokio::main]
async fn main() {
    let Cli {
        address,
        command,
        file,
        directory,
        arguments,
        ..
    } = Cli::parse();

    let ip = Ipv4Addr::new(0, 0, 0, 0);

    match &command {
        Some(Commands::Server { port }) => start_server(ip, *port).await,

        None => {
            if is_container() {
                start_server(ip, 39493).await;

                return;
            }

            if let Some(file) = file {
                let mut smart_directory = String::new();

                if let Ok(file) = file.canonicalize() {
                    let components = &mut file.components();
                    let mut i = 0;

                    for component in components.by_ref() {
                        if component == Component::RootDir {
                            continue;
                        }

                        smart_directory = format!(
                            "{}/{}",
                            smart_directory,
                            component.as_os_str().to_str().unwrap()
                        );

                        i += 1;

                        if i == 3 {
                            break;
                        }
                    }
                } else {
                    smart_directory = file.to_str().unwrap().to_string();
                }


                if let Some(directory) = directory {
                    client::exec(address, &file, &arguments, directory.to_str().unwrap());
                } else {
                    client::exec(address, &file, &arguments, &smart_directory);
                }

                return;
            }

            notify("sakaya was called but no file was given.", None);
        }
    }
}

async fn start_server(ip: Ipv4Addr, port: u16) {
    let running_ip = SocketAddrV4::new(ip, port);

    server::serve(running_ip).await;
}
