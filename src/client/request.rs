use std::net::SocketAddrV4;

use crate::state::Options;

/// Sends a request to start an application inside a container
pub fn request(
    address: SocketAddrV4,
    path: &str,
    wine_prefix: &str,
    arguments: &Vec<String>,
    command: &str,
) -> Result<(), minreq::Error> {
    let opts = Options::new(path, wine_prefix, arguments);

    let url = format!("http://{address}/{command}");
    let response = minreq::post(url).with_json(&opts)?.send()?;

    print!("{}", response.as_str()?);

    Ok(())
}
