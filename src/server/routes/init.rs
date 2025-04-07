use axum::extract::Json;
use std::{path::Path, process::Command};

use crate::{state::Options, util::notify};

/// Create a wine prefix
pub async fn init(Json(options): Json<Options>) -> Result<String, &'static str> {
    let envs = Options::vars(&options);

    if Path::new(&options.wine_prefix).exists() {
        return Ok("Prefix already exists.\n".to_string());
    }

    notify("Initializing wine prefix...", None);

    Command::new("wineboot").envs(&envs).output().unwrap();

    Ok("Created successfully.\n".to_string())
}
