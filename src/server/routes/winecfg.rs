use axum::extract::Json;
use std::process::{Command, Output};

use crate::state::Options;

pub async fn winecfg(Json(options): Json<Options>) -> Result<String, &'static str> {
    let Ok(Output { stdout, stderr, .. }) = Command::new("winecfg")
        .envs(Options::vars(&options))
        .arg(options.path)
        .args(options.arguments)
        .output()
    else {
        return Err("Error while trying to run winecfg.");
    };

    let Ok(stdout) = String::from_utf8(stdout) else {
        return Err("winecfg returned invalid stdout.");
    };

    let Ok(stderr) = String::from_utf8(stderr) else {
        return Err("winecfg returned invalid stderr.");
    };

    Ok(format!("stdout:\n{stdout}\nstderr:\n{stderr}"))
}
