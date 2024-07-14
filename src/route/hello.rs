use axum::response::IntoResponse;


pub(crate) async fn hello_world() -> impl IntoResponse {
    "Hello, world!"
}