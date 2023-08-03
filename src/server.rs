use rocket::serde::{json::Json, Deserialize};
use rocket::{post, routes};
use std::net::{IpAddr, Ipv4Addr};
use std::process::Command;

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

pub async fn rocket() {
    let host_ip_from_container = IpAddr::V4(Ipv4Addr::new(192, 168, 100, 49));

    let figment = rocket::Config::figment()
        .merge(("port", 39493))
        .merge(("address", host_ip_from_container));

    let _ = rocket::custom(figment)
        .mount("/", routes![post])
        .launch()
        .await;
}
