use crate::cli::Cli;
use crate::util::notify;

use clap::Parser;
use std::net::SocketAddrV4;
use std::path::Path;

use super::{get_target_machine, make_desktop_file, make_icon, make_xauth, request};

/// Run an executable inside the container from the host by requesting
/// the server on a given socket address
pub fn exec(address: SocketAddrV4, path: &Path, arguments: &Vec<String>, directory: &str) {
    if !path.exists() {
        notify("Exiting since not a valid file.", None);
        return;
    }

    let ping = minreq::get(format!("http://{address}/"))
        .with_timeout(1)
        .send();

    if ping.is_err() {
        notify(
            &format!("Error: sakaya server is not accessible on {address}."),
            None,
        );

        return;
    }

    if let Some(session) = std::env::var_os("XDG_SESSION_TYPE") {
        if session == "x11" {
            make_xauth();
        }
    }

    let file_name = path.file_name().unwrap().to_str().unwrap();
    let path = path.canonicalize().unwrap();
    let path = path.to_str().unwrap();

    let Cli { wine32, wine64, .. } = Cli::parse();

    if path.contains(directory) {
        let container_path = path.replace(directory, "mnt");
        let icon = make_icon(path, file_name);

        let wine_prefix = match get_target_machine(path) {
            32 => &wine32,
            64 => &wine64,
            _ => "",
        };

        if wine_prefix.is_empty() {
            notify(
                "Exiting since 32/64-bit could not be determined. Please report this issue.",
                None,
            );
            return;
        }

        request(address, &container_path, wine_prefix, arguments, "init").unwrap();

        make_desktop_file(file_name, path);

        notify(&format!("Starting {file_name} with {wine_prefix}..."), Some(&icon));

        if request(address, &container_path, wine_prefix, arguments, "open").is_ok() {
            notify(&format!("Closed {file_name}."), Some(&icon));
        } else {
            notify("Error: sakaya server is not accessible.", None);
        }
    }
}
