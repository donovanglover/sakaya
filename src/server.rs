use std::io::prelude::*;
use std::io::BufReader;
use std::net::SocketAddrV4;
use std::net::{TcpListener, TcpStream};
use std::process::{Command, Output};
use std::thread;
use urlencoding::decode;

/// Simple HTTP server that opens files based on GET requests
pub fn start(address: SocketAddrV4) {
    let listener = TcpListener::bind(address).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        thread::spawn(|| {
            handle_connection(stream);
        });
    }
}

/// Opens the given file and returns 200, otherwise 404
fn handle_connection(mut stream: TcpStream) {
    let request_line = BufReader::new(&mut stream).lines().next().unwrap().unwrap();
    let request_line: Vec<&str> = request_line.split(' ').collect();

    if let Some(request) = request_line.get(1) {
        out(stream, "HTTP/1.1 200 OK", &open(request));
    } else {
        out(stream, "HTTP/1.1 404 NOT FOUND", "");
    }
}

/// Handles outputting to the requester
fn out(mut stream: TcpStream, status: &str, contents: &str) {
    let length = contents.len();
    let response = format!("{status}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}

/// Open an executable in wine
fn open(request: &str) -> String {
    let request = decode(request).unwrap();

    println!("{}", request);

    let split = request.split("//").collect::<Vec<_>>();

    let Output { stdout, stderr, .. } = Command::new("wine")
        .env("WINEPREFIX", split[1])
        .arg(split[0])
        .output()
        .unwrap();

    let stdout = String::from_utf8(stdout).unwrap();
    let stderr = String::from_utf8(stderr).unwrap();

    format!("stdout:\n{stdout}\nstderr:\n{stderr}")
}
