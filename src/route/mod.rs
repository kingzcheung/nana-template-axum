use axum::{routing::get, Router};
use hello::hello_world;
use hyper::Method;
use tower_http::cors::{Any, CorsLayer};

use crate::state::AppState;

pub mod hello;

pub fn routes() -> axum::Router<AppState> {
    Router::new()
    .route("/", get(hello_world))
    // more routes init here
    .layer(
        CorsLayer::new()
            .allow_origin(Any)
            .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
            .allow_headers(Any),
    )
}