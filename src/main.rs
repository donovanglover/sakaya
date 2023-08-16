use clap::Parser;
use cli::Cli;
use home::home_dir;
use sakaya::*;
use std::path::Path;

mod cli;

fn main() {
    let cli = Cli::parse();

    if is_container() {
        server("127.0.0.1:7878");
        // server("192.168.100.49:39493");

        return;
    }

    // TOOD: DRY
    if &cli.executable == "winecfg" {
        request("winecfg").unwrap();
        notify("Closed winecfg.", None);
    }

    let path = Path::new(&cli.executable);

    if !path.exists() {
        println!("File is NOT in path");
        return;
    }

    let full_path = path.canonicalize().unwrap();
    let full_path_str = full_path.to_str().unwrap();

    let file_name_str = match full_path.file_name() {
        Some(file_name) => file_name.to_str().unwrap(),
        None => "",
    };

    // TODO: Don't hardcode this?
    if full_path_str.contains("/home/user/containers/wine") {
        let container_path = full_path_str.replace("/home/user/containers/wine", "/mnt");

        let _home = home_dir().unwrap();
        let home = _home.to_str().unwrap();

        let icon_path = &format!("{home}/.local/share/icons/{file_name_str}.png");
        let desktop_file_path =
            &format!("{home}/.local/share/applications/{file_name_str}.desktop");

        make_icon(full_path_str, icon_path);
        make_desktop_file(desktop_file_path, file_name_str, full_path_str);
        notify(&format!("Starting {file_name_str}..."), Some(icon_path));
        request(&container_path).unwrap();
        notify(&format!("Closed {file_name_str}."), Some(icon_path));
    }
}
