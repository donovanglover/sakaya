use axum::extract::Json;
use std::process::{Command, Output};

use crate::state::Options;

/// Open an executable in wine
pub async fn open(Json(options): Json<Options>) -> Result<String, &'static str> {
    let Ok(Output { stdout, stderr, .. }) = Command::new("wine")
        .env("WINEPREFIX", options.wine_prefix)
        .env("WAYLAND_DISPLAY", options.wayland_display)
        .env("XDG_RUNTIME_DIR", options.xdg_runtime_dir)
        .env("DISPLAY", options.display)
        .env("XAUTHORITY", "/tmp/.X11-unix/Xauthority")
        .env("LC_ALL", options.locale)
        .env("TZ", options.timezone)
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
