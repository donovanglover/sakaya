use serde::Deserialize;
use serde::Serialize;
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
}

impl Options {
    pub fn new(path: &str, wine_prefix: &str) -> Self {
        Self {
            path: path.to_string(),
            wine_prefix: wine_prefix.to_string(),
            ..Default::default()
        }
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
        }
    }
}
