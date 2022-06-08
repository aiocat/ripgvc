use axum::{
    routing::{get, post},
    http::StatusCode,
    response::IntoResponse,
    Router,
};
use std::net::SocketAddr;
mod drawing;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root));

    let port = std::env::var("PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(3000);

    let address = SocketAddr::from(([0, 0, 0, 0], port));
    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// basic handler that responds with a static string
async fn root() -> impl IntoResponse {
    ([("content-type", "image/svg+xml")], drawing::draw_file("unknown hex value!!!"))
}