use clap::Parser;
use home::home_dir;
use notify_rust::Notification;
use reqwest::blocking::ClientBuilder;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

mod cli;
mod server;
mod writer;

#[rocket::main]
async fn main() {
    let cli = cli::Cli::parse();

    let is_container = fs::read("/run/systemd/container").is_ok();

    if is_container {
        server::rocket().await;
        return
    }

    // TOOD: DRY
    if &cli.executable == "winecfg" {
        let mut map = HashMap::new();
        map.insert("wine", "/mnt/.winevn-win32-wow-dotnet40-breeze-dark");
        map.insert("path", "winecfg");
        let client = ClientBuilder::new().timeout(None).build().unwrap();
        let result = client
            .post("http://192.168.100.49:39493")
            .json(&map)
            .send()
            .expect("Couldn't request sakaya-server")
            .text();

        let log_file: String = "/tmp/sakaya-winecfg.log".to_owned();

        let _ = fs::write(&log_file, result.unwrap());

        println!("Log file available at {}", log_file)
    }

    let path = Path::new(&cli.executable);

    if !path.exists() {
        println!("File is NOT in path");
        return
    }

    let full_path = path.canonicalize().unwrap();
    let full_path_str = full_path.to_str().expect("Couldn't convert to str");

    let file_name_str = match full_path.file_name() {
        Some(file_name) => file_name.to_str().expect("Couldn't convert to str"),
        None => "",
    };

    // TODO: Don't hardcode this?
    if full_path_str.contains("/home/user/containers/wine") {
        let container_path = full_path_str.replace("/home/user/containers/wine", "/mnt");
        let path_str = path.to_str().expect("Couldn't convert to str");

        println!("Running {} as {}...", path_str, container_path);

        let mut map = HashMap::new();
        map.insert("wine", "/mnt/.winevn-win32-wow-dotnet40-breeze-dark");
        map.insert("path", &container_path);

        let mut starting_string: String = "Starting ".to_owned();
        starting_string.push_str(file_name_str);
        starting_string.push_str("...");

        let home = home_dir().unwrap();
        let home_result = home.to_str().unwrap();

        let icon_path =
            &(home_result.to_owned() + "/.local/share/icons/" + file_name_str + ".png");
        let desktop_file_path = &(home_result.to_owned()
            + "/.local/share/applications/"
            + file_name_str
            + ".desktop");

        writer::make_icon(full_path_str, icon_path);
        writer::make_desktop_file(desktop_file_path, file_name_str, full_path_str);

        let _ = Notification::new()
            .summary("酒屋")
            .body(&starting_string)
            .icon(icon_path)
            .timeout(3000)
            .show();

        let client = ClientBuilder::new().timeout(None).build().unwrap();
        let result = client
            .post("http://192.168.100.49:39493")
            .json(&map)
            .send()
            .expect("Couldn't request sakaya-server")
            .text();

        let mut log_file: String = "/tmp/sakaya-".to_owned();
        log_file.push_str(file_name_str);
        log_file.push_str(".log");

        let _ = fs::write(&log_file, result.unwrap());

        println!("Log file available at {}", log_file)
    }
}
