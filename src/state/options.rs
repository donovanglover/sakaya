use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
pub struct Options {
    pub wine_prefix: String,
    pub path: String,
}
