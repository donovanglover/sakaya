use axum::extract::Json;
use std::process::{Command, Output};

use crate::state::Options;

/// Open an executable in wine
pub async fn open(Json(options): Json<Options>) -> Result<String, &'static str> {
    let Ok(Output { stdout, stderr, .. }) = Command::new("wine")
        .env("WINEPREFIX", options.wine_prefix)
        .env("WAYLAND_DISPLAY", "wayland-1")
        .env("XDG_RUNTIME_DIR", "/run/user/1000")
        .env("DISPLAY", ":0")
        .env("XAUTHORITY", "/tmp/.X11-unix/Xauthority")
        .arg(options.path)
        .output()
    else {
        return Err("Error while trying to run the wine command.");
    };

    let Ok(stdout) = String::from_utf8(stdout) else {
        return Err("The program returned invalid stdout.");
    };

    let Ok(stderr) = String::from_utf8(stderr) else {
        return Err("The program returned invalid stderr.");
    };

    Ok(format!("stdout:\n{stdout}\nstderr:\n{stderr}"))
}
