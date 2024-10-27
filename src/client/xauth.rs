use std::process::{Command, Stdio};
use std::env::var;

pub fn make_xauth() {
    let display = var("DISPLAY").unwrap();
    let xauth_file = "/tmp/.X11-unix/Xauthority";

    let xauth_child = Command::new("xauth")
        .arg("nextract")
        .arg("-")
        .arg(display)
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let sed_child = Command::new("sed")
        .arg("-e")
        .arg("s/^..../ffff/")
        .stdin(Stdio::from(xauth_child.stdout.unwrap()))
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    Command::new("xauth")
        .arg("-f")
        .arg(xauth_file)
        .arg("nmerge")
        .arg("-")
        .stdin(Stdio::from(sed_child.stdout.unwrap()))
        .stdout(Stdio::piped())
        .spawn()
        .unwrap()
        .wait_with_output()
        .unwrap();
}
