use rocket::{get, post, launch, routes};
use rocket::serde::{Deserialize, json::Json};
use std::process::Command;
use std::str;

#[get("/")]
fn get() -> String {
    let output = Command::new("/usr/bin/env")
        .arg("ps")
        .output()
        .expect("Failed to execute command");

    let result = match str::from_utf8(&output.stdout).ok() {
        Some(string) => string,
        None => ""
    };

    format!("{:?}", result)
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct MyCommand<'r> {
    path: &'r str,
    wine: &'r str
}

#[post("/", data = "<data>")]
fn post(data: Json<MyCommand<'_>>) -> String {
    let output = Command::new("wine")
        .env("WINEPREFIX", data.wine)
        .arg(data.path)
        .output()
        .expect("Failed to execute command");

    format!("{:?}", output)
}

#[launch]
fn rocket() -> _ {
    let virt = Command::new("systemd-detect-virt")
        .output()
        .expect("Failed to detect");

    if String::from_utf8_lossy(&virt.stdout) != "kvm" {
        println!("WARNING: sakaya-server was NOT executed inside of a systemd-nspawn container.");
    }

    rocket::build().mount("/", routes![
        get,
        post
    ])
}
