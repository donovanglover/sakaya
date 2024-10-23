use axum::extract::Json;
use axum::routing::post;
use axum::Router;
use serde::Deserialize;
use serde::Serialize;
use std::net::SocketAddrV4;
use std::process::{Command, Output};

#[derive(Serialize, Deserialize)]
pub struct Options {
    pub wine_prefix: String,
    pub path: String,
}

pub async fn start(address: SocketAddrV4) {
    let app = Router::new().route("/open", post(open_executable));
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

/// Open an executable in wine
async fn open_executable(Json(options): Json<Options>) -> Result<String, &'static str> {
    let Ok(Output { stdout, stderr, .. }) = Command::new("wine")
        .env("WINEPREFIX", options.wine_prefix)
        .env("WAYLAND_DISPLAY", "wayland-1")
        .env("XDG_RUNTIME_DIR", "/run/user/1000")
        .env("DISPLAY", ":0")
        .env("XAUTHORITY", "/tmp/container_xauth")
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
