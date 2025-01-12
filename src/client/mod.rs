use crate::cli::Cli;
use crate::state::Options;
use crate::util::notify;
use clap::Parser;
use std::net::SocketAddrV4;
use std::path::Path;

pub mod desktop;
pub mod get_target_machine;
pub mod icon;
pub mod xauth;
pub use desktop::*;
pub use get_target_machine::*;
pub use icon::*;
pub use xauth::*;

/// Run an executable inside the container from the host by requesting
/// the server on a given socket address
pub fn exec(address: SocketAddrV4, path: &Path, directory: &str) {
    if !path.exists() {
        notify("Exiting since not a valid file.", None);
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

    if path.contains(directory) {
        let container_path = path.replace(directory, "mnt");
        let icon = make_icon(path, file_name);

        let Cli { wine32, wine64, .. } = Cli::parse();
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

        if !Path::new(wine_prefix).exists() {
            request(address, &container_path, wine_prefix, "init").unwrap();
        }

        make_desktop_file(file_name, path);

        notify(&format!("Starting {file_name}..."), Some(&icon));

        if request(address, &container_path, wine_prefix, "open").is_ok() {
            notify(&format!("Closed {file_name}."), Some(&icon));
        } else {
            notify("Error: sakaya server is not accessible.", None);
        }
    }
}

/// Sends a request to start an application inside a container
pub fn request(
    address: SocketAddrV4,
    path: &str,
    wine_prefix: &str,
    command: &str,
) -> Result<(), minreq::Error> {
    let opts = Options::new(path, wine_prefix);

    let url = format!("http://{address}/{command}");
    let response = minreq::post(url).with_json(&opts)?.send()?;

    print!("{}", response.as_str()?);

    Ok(())
}
