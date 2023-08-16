use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use std::io::BufReader;
use std::process::Command;
use std::thread;

fn main() {
    server("127.0.0.1:7878")
}

/// Simple HTTP server that opens files based on GET requests
fn server(address: &str) {
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
        open(request);
        out(stream, "HTTP/1.1 200 OK", "");
    }

    // out(stream, "HTTP/1.1 404 NOT FOUND", "");
}

/// Handles outputting to the requester
fn out(mut stream: TcpStream, status: &str, contents: &str) {
    let length = contents.len();
    let response = format!("{status}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}

fn open(request: &str) {
    let output = Command::new("wine")
        // .env("WINEPREFIX", &data.wine)
        .arg(request)
        .output()
        .expect("Failed to execute command");

    println!("{:?}", output);
}
