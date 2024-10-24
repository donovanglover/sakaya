use axum::routing::post;
use axum::Router;

use super::routes;

pub fn router() -> Router {
    Router::new().route("/open", post(routes::open))
}
