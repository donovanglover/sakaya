use rocket::{get, post, launch, routes};
use rocket::serde::{Deserialize, json::Json};
use std::process::Command;

#[get("/")]
fn get() -> String {
    format!("Hello!")
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct MyCommand<'r> {
    path: &'r str
}

#[post("/", data = "<data>")]
fn post(data: Json<MyCommand<'_>>) -> String {
    format!("{}", data.path)
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
