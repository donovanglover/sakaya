use std::fs;
use std::process::Command;

/// Checks if we're inside a container
pub fn is_container() -> bool {
    fs::read("/run/systemd/container").is_ok()
}

/// Notifies the user of an event
pub fn notify(body: &str, mut icon: Option<&str>) {
    println!("{body}");

    if !is_container() {
        Command::new("dunstify")
            .args([
                "--icon",
                icon.get_or_insert("sakaya"),
                "--timeout",
                "3000",
                "酒屋",
                body,
            ])
            .output()
            .unwrap();
    }
}
