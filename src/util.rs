use notify_rust::Notification;
use std::fs;

/// Checks if we're inside a container
pub fn is_container() -> bool {
    fs::read("/run/systemd/container").is_ok()
}

/// Notifies the user of an event
pub fn notify(body: &str, mut icon: Option<&str>) {
    println!("{body}");

    if !is_container() {
        Notification::new()
            .summary("酒屋")
            .body(body)
            .icon(icon.get_or_insert("sakaya"))
            .timeout(3000)
            .show()
            .unwrap();
    }
}
