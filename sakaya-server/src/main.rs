use rocket::{get, post, launch, routes};
use rocket::serde::{Deserialize, json::Json};
use std::process::Command;
use std::str;
use std::net::{IpAddr, Ipv4Addr};

#[get("/")]
fn get() -> String {
    let output = Command::new("/usr/bin/env")
        .arg("ps")
        .output()
        .expect("Failed to execute command");

    let result = str::from_utf8(&output.stdout).ok().unwrap_or("");

    format!("{:?}", result)
}

#[derive(Deserialize)]
struct MyCommand {
    path: String,
    wine: String
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

#[launch]
fn rocket() -> _ {
    let host_ip_from_container = IpAddr::V4(Ipv4Addr::new(192, 168, 100, 49));

    let figment = rocket::Config::figment()
        .merge(("port", 39493))
        .merge(("address", host_ip_from_container));

    rocket::custom(figment).mount("/", routes![
        get,
        post
    ])
}
