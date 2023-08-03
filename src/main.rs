use clap::Parser;
use home::home_dir;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

mod cli;
mod server;
mod client;

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

        client::log("winecfg", &client::request(&map));
        client::notify("Closed winecfg.", None);
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

        println!("Running {path_str} as {container_path}...");

        let mut map = HashMap::new();
        map.insert("wine", "/mnt/.winevn-win32-wow-dotnet40-breeze-dark");
        map.insert("path", &container_path);

        let home = home_dir().unwrap();
        let home_result = home.to_str().unwrap();

        let icon_path =
            &(home_result.to_owned() + "/.local/share/icons/" + file_name_str + ".png");
        let desktop_file_path = &(home_result.to_owned()
            + "/.local/share/applications/"
            + file_name_str
            + ".desktop");

        client::make_icon(full_path_str, icon_path);
        client::make_desktop_file(desktop_file_path, file_name_str, full_path_str);
        client::notify(&format!("Starting {file_name_str}..."), Some(icon_path));
        client::log(file_name_str, &client::request(&map));
        client::notify(&format!("Closed {file_name_str}."), Some(icon_path));
    }
}
