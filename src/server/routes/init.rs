use axum::extract::Json;
use std::process::Command;

use crate::{state::Options, util::notify};

/// Create a wine prefix
pub async fn init(Json(options): Json<Options>) -> Result<String, &'static str> {
    notify("Initializing wine prefix...", None);

    Command::new("wineboot")
        .envs(Options::vars(&options))
        .output()
        .unwrap();

    Command::new("winetricks")
        .arg("fontsmooth=rgb")
        .envs(Options::vars(&options))
        .output()
        .unwrap();

    Command::new("winetricks")
        .arg("-q")
        .args([
            "dotnet35",
            "vcrun2003",
            "vcrun2005",
            "vcrun2008",
            "vcrun2010",
            "vcrun2012",
            "vcrun2013",
            "vcrun2015",
            "lavfilters",
            "alldlls=default",
            "quartz",
            "dxvk",
        ])
        .envs(Options::vars(&options))
        .output()
        .unwrap();

    Command::new("winetricks")
        .arg("-q")
        .arg("wmp10")
        .envs(Options::vars(&options))
        .output()
        .unwrap();

    Command::new("winetricks")
        .arg("renderer=gdi")
        .envs(Options::vars(&options))
        .output()
        .unwrap();

    Ok("Created successfully.".to_string())
}
