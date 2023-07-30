use rocket::{get, post, launch, routes};
use std::process::Command;

#[get("/")]
fn get() -> String {
    format!("Hello!")
}

#[post("/", data = "<data>")]
fn post(data: &str) -> String {
    format!("{}", data)
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
