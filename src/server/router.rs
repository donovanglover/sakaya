use axum::routing::{get, post};
use axum::Router;

use super::routes;

pub fn router() -> Router {
    Router::new()
        .route("/", get(routes::index))
        .route("/open", post(routes::open))
        .route("/init", post(routes::init))
        .route("/winecfg", post(routes::winecfg))
}
