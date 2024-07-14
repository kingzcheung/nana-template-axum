use std::net::SocketAddr;

use axum::{body::Body, extract::Request, response::IntoResponse, Router};
use hyper::Uri;
use tracing::info;

use crate::{config::Config, route::routes, state::AppState};

pub async fn serve(app: Router, port: u16) {
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    info!("server listening on {}, see: http://{}", addr, addr);
    axum::serve(listener, app)
        // .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

pub async fn start(conf: Config) {
    let port = conf.server_port;

    let shared_state = AppState { config: conf };

    let app = Router::new()
        .nest("/api", routes())
        .with_state(shared_state)
        .fallback(index_handler);

    serve(app, port).await;
}

pub async fn index_handler(request: Request<Body>) -> impl IntoResponse {
    handler_404(request.uri()).await
}

async fn handler_404(_uri: &Uri) -> impl IntoResponse {
    "404 Not Found"
}
