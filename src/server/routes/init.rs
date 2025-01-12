use axum::extract::Json;
use std::{collections::HashMap, path::Path, process::Command};

use crate::{state::Options, util::notify};

/// Create a wine prefix
pub async fn init(Json(options): Json<Options>) -> Result<String, &'static str> {
    let envs = Options::vars(&options);

    if Path::new(&options.wine_prefix).exists() {
        return Ok("Prefix already exists.".to_string());
    }

    notify("Initializing wine prefix...", None);

    Command::new("wineboot").envs(&envs).output().unwrap();

    let commands = [
        "fontsmooth=rgb",
        "dotnet35",
        "dotnet40",
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
        "wmp10",
        "renderer=gdi",
    ];

    winetricks(&commands, envs);

    Ok("Created successfully.".to_string())
}

fn winetricks(commands: &[&str], envs: HashMap<&str, String>) {
    let len = commands.len();
    let mut i = 1;

    for command in commands {
        notify(
            &format!("Running winetricks {command}... ({i}/{len})"),
            None,
        );

        Command::new("winetricks")
            .arg("-q")
            .arg(command)
            .envs(&envs)
            .output()
            .unwrap();

        i += 1;
    }
}
