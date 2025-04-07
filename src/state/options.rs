use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::env::var;

use crate::consts::DEFAULT_LOCALE;
use crate::consts::DEFAULT_TIMEZONE;
use crate::consts::DEFAULT_WINE32_PREFIX;

#[derive(Serialize, Deserialize)]
pub struct Options {
    pub wine_prefix: String,
    pub path: String,
    pub wayland_display: String,
    pub xdg_runtime_dir: String,
    pub display: String,
    pub locale: String,
    pub timezone: String,
    pub arguments: Vec<String>,
}

impl Options {
    pub fn new(path: &str, wine_prefix: &str, arguments: &[String]) -> Self {
        Self {
            path: path.to_string(),
            wine_prefix: wine_prefix.to_string(),
            arguments: arguments.to_vec(),
            ..Default::default()
        }
    }

    pub fn vars(&self) -> HashMap<&str, String> {
        let wine_arch = match self.wine_prefix.contains("64") {
            true => "win64",
            false => "win32",
        };

        HashMap::from([
            ("WINEPREFIX", self.wine_prefix.to_string()),
            ("WAYLAND_DISPLAY", self.wayland_display.to_string()),
            ("XDG_RUNTIME_DIR", self.xdg_runtime_dir.to_string()),
            ("DISPLAY", self.display.to_string()),
            ("XAUTHORITY", "/tmp/.X11-unix/Xauthority".to_string()),
            ("LC_ALL", self.locale.to_string()),
            ("TZ", self.timezone.to_string()),
            ("WINEARCH", wine_arch.to_string()),
        ])
    }
}

impl Default for Options {
    fn default() -> Self {
        Self {
            wine_prefix: DEFAULT_WINE32_PREFIX.to_string(),
            path: "/tmp/sakaya".to_string(),
            wayland_display: var("WAYLAND_DISPLAY").unwrap_or_default(),
            xdg_runtime_dir: var("XDG_RUNTIME_DIR").unwrap(),
            display: var("DISPLAY").unwrap(),
            locale: DEFAULT_LOCALE.to_string(),
            timezone: DEFAULT_TIMEZONE.to_string(),
            arguments: vec![],
        }
    }
}
