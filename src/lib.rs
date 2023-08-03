use std::fs;

pub fn is_container() -> bool {
    fs::read("/run/systemd/container").is_ok()
}
