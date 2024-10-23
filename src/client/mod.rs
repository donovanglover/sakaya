use crate::cli::Cli;
use crate::server::Options;
use crate::util::notify;
use clap::Parser;
use pelite::{FileMap, PeFile};
use std::net::SocketAddrV4;
use std::path::Path;

mod desktop;
mod icon;
pub use desktop::*;
pub use icon::*;

/// https://github.com/MicrosoftDocs/win32/blob/docs/desktop-src/Debug/pe-format.md#machine-types
const IMAGE_FILE_MACHINE_I386: u16 = 0x14C;
const IMAGE_FILE_MACHINE_AMD64: u16 = 0x8664;

/// Run an executable inside the container from the host by requesting
/// the server on a given socket address
pub fn exec(address: SocketAddrV4, path: &Path, directory: &str) {
    if !path.exists() {
        notify("Exiting since not a valid file.", None);
        return;
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

        make_desktop_file(file_name, path);

        notify(&format!("Starting {file_name}..."), Some(&icon));

        if request(address, &container_path, wine_prefix).is_ok() {
            notify(&format!("Closed {file_name}."), Some(&icon));
        } else {
            notify("Error: sakaya server is not accessible.", None);
        }
    }
}

/// Sends a request to start an application inside a container
pub fn request(address: SocketAddrV4, path: &str, wine_prefix: &str) -> Result<(), minreq::Error> {
    let opts = Options {
        path: path.to_string(),
        wine_prefix: wine_prefix.to_string(),
    };

    let url = format!("http://{address}/open");
    let response = minreq::post(url).with_json(&opts)?.send()?;

    print!("{}", response.as_str()?);

    Ok(())
}

/// Gets whether the exe is 32 or 64-bit
pub fn get_target_machine(input_bin: &str) -> u8 {
    let map = FileMap::open(input_bin).expect("Error opening the binary");
    let file = PeFile::from_bytes(&map).expect("Error parsing the binary");
    let target_machine = file.file_header().Machine;

    match target_machine {
        IMAGE_FILE_MACHINE_I386 => 32,
        IMAGE_FILE_MACHINE_AMD64 => 64,
        _ => 0,
    }
}
