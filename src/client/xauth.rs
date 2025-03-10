use std::env::var;
use std::process::{Command, Stdio};

pub fn make_xauth() {
    let display = var("DISPLAY").unwrap();
    let xauth_file = "/tmp/.X11-unix/Xauthority";

    let xauth_child = Command::new("xauth")
        .arg("nextract")
        .arg("-")
        .arg(display)
        .stdout(Stdio::piped())
        .spawn()
        .unwrap()
        .stdout
        .unwrap();

    let sed_child = Command::new("sed")
        .arg("-e")
        .arg("s/^..../ffff/")
        .stdin(Stdio::from(xauth_child))
        .stdout(Stdio::piped())
        .spawn()
        .unwrap()
        .stdout
        .unwrap();

    Command::new("xauth")
        .arg("-f")
        .arg(xauth_file)
        .arg("nmerge")
        .arg("-")
        .stdin(Stdio::from(sed_child))
        .stdout(Stdio::piped())
        .spawn()
        .unwrap()
        .wait_with_output()
        .unwrap();
}
