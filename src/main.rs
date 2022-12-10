// Run with `RUST_LOG=debug cargo run`

use axum::{
    response::{Html, Response, IntoResponse}, 
    routing::{get, post},
    http::{StatusCode},
    Router,
};
use std::net::SocketAddr;
use tracing::{debug};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    // A layer that logs events to stdout using the human-readable "pretty"
    // format.
    // let stdout_log = tracing_subscriber::fmt::layer()
    //     .pretty();

    // build our application with a route
    let app = Router::new()
        .route("/", get(handler))
        .route("/my_event", post(hook_handler))
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

async fn hook_handler() -> Response {
    debug!("Received request");
    (StatusCode::OK, "success").into_response()
}