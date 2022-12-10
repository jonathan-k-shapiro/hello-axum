// Run with `RUST_LOG=debug cargo run`

use axum::{
    body::Bytes,
    http::{HeaderMap, StatusCode},
    response::{Html, Response, IntoResponse}, 
    Router,
    routing::{get, post},
};
use std::net::SocketAddr;
use tracing::{debug};

use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        .route("/", get(handler))
        .route("/my_event", post(hook_handler))
        .layer(TraceLayer::new_for_http())
    ;

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

async fn hook_handler(headers: HeaderMap, body: Bytes) -> Response {
    debug!(?headers, ?body, "Received request");
    (StatusCode::OK, "success").into_response()
}