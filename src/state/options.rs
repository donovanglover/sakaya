use serde::Deserialize;
use serde::Serialize;

use crate::consts::DEFAULT_WINE32_PREFIX;

#[derive(Serialize, Deserialize)]
pub struct Options {
    pub wine_prefix: String,
    pub path: String,
    pub display: String,
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
            display: std::env::var_os("DISPLAY").unwrap().into_string().unwrap(),
        }
    }
}
