use std::fs;
use std::io::Cursor;
use pelite::{FileMap, PeFile};
use std::process::Command;
use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use std::io::BufReader;
use std::thread;
use minreq;

/// Checks if we're inside a container
pub fn is_container() -> bool {
    fs::read("/run/systemd/container").is_ok()
}

/// Given an .exe file, return the first .ico file inside it
pub fn get_first_ico_file(input_bin: &str) -> Option<Cursor<Vec<u8>>> {
    let map = FileMap::open(input_bin).expect("Error opening the binary");
    let file = PeFile::from_bytes(&map).expect("Error parsing the binary");
    let resources = file.resources().expect("Error binary does not have resources");

    for (_, group) in resources.icons().filter_map(Result::ok) {
        let mut contents = Vec::new();
        group.write(&mut contents).unwrap();

        return Some(Cursor::new(contents));
    }

    None
}

/// Given an .ico with multiple images, return the largest one that's a square
pub fn convert_largest_square_image_in_ico_to_png(buf: Cursor<Vec<u8>>) {
    let icondir = ico::IconDir::read(buf).unwrap();
    let image = icondir.entries()[3].decode().unwrap();
    let out_file = fs::File::create(format!("result.png")).unwrap();

    image.write_png(out_file).unwrap();
}

/// Makes an icon for the application with icoextract
pub fn make_icon(input_path: &str, output_icon: &str) {
    Command::new("icoextract")
        .arg(input_path)
        .arg(output_icon)
        .output()
        .unwrap();
}

/// Makes a desktop file for the application
pub fn make_desktop_file(output_location: &str, file_name: &str, full_path: &str) {
    let mut output: String = "[Desktop Entry]".to_owned() + "\n";
    output.push_str("Type=Application\n");
    output.push_str(&("Name=".to_owned() + file_name + "\n"));
    output.push_str(&("Exec=sakaya \"".to_owned() + full_path + "\"\n"));

    let _ = fs::write(output_location, output);
}

/// Notifies the user of an event
pub fn notify(body: &str, mut icon: Option<&str>) {
    println!("{body}");
    Command::new("dunstify")
        .args(["--icon", icon.get_or_insert("sakaya"), "--timeout", "3000", "酒屋", body])
        .output()
        .unwrap();
}

/// Sends a request to start an application inside a container
pub fn request(path: &str) -> Result<(), minreq::Error> {
    // http://192.168.100.49:39493
    let o = minreq::get(format!("http://127.0.0.1:7878/{path}")).send()?;
    let s = o.as_str()?;
    print!("{}", s);
    Ok(())
}

/// Simple HTTP server that opens files based on GET requests
pub fn server(address: &str) {
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
