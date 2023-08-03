use clap::Parser;
use home::home_dir;
use notify_rust::Notification;
use reqwest::blocking::ClientBuilder;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::process::Command;
use rocket::serde::{json::Json, Deserialize};
use rocket::{post, routes};
use std::net::{IpAddr, Ipv4Addr};

#[derive(Deserialize)]
struct MyCommand {
    path: String,
    wine: String,
}

#[post("/", data = "<data>")]
fn post(data: Json<MyCommand>) -> String {
    let output = Command::new("wine")
        .env("WINEPREFIX", &data.wine)
        .arg(&data.path)
        .output()
        .expect("Failed to execute command");

    format!("{:?}", output)
}

async fn rocket() {
    let host_ip_from_container = IpAddr::V4(Ipv4Addr::new(192, 168, 100, 49));

    let figment = rocket::Config::figment()
        .merge(("port", 39493))
        .merge(("address", host_ip_from_container));

    let _ = rocket::custom(figment)
        .mount("/", routes![post])
        .launch()
        .await;
}

#[derive(Parser)]
#[command(version)]
#[command(arg_required_else_help = true)]
struct Cli {
    /// Path to the executable to run.
    executable: String,
}

fn make_icon(input_path: &str, output_icon: &str) {
    Command::new("icoextract")
        .arg(input_path)
        .arg(output_icon)
        .output()
        .expect("failed to execute process");
}

fn make_desktop_file(output_location: &str, file_name: &str, full_path: &str) {
    let mut output: String = "[Desktop Entry]".to_owned() + "\n";
    output.push_str("Type=Application\n");
    output.push_str(&("Name=".to_owned() + file_name + "\n"));
    output.push_str(&("Exec=sakaya \"".to_owned() + full_path + "\"\n"));

    let _ = fs::write(output_location, output);
}


#[rocket::main]
async fn main() {
    // rocket().await;
    let cli = Cli::parse();

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

    if path.exists() {
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

            make_icon(full_path_str, icon_path);
            make_desktop_file(desktop_file_path, file_name_str, full_path_str);

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
        } else {
            // TODO: Get rid of else statements
            println!("File is NOT in path")
        }
    }
}
