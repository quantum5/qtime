use axum::Router;
use axum::routing::get;
use crate::{iso8601, unix};

pub fn app() -> Router {
    Router::new()
        .route("/unix", get(unix::unix))
        .route("/unix_dec", get(unix::unix_dec))
        .route("/unix_ms", get(unix::unix_ms))
        .route("/iso8601", get(iso8601::iso8601))
        .route("/iso8601_ms", get(iso8601::iso8601_ms))
}
