// Run with `RUST_LOG=debug cargo run`

use axum::{
    // body::Bytes,
    http::{HeaderMap, StatusCode},
    response::{Html, Response, IntoResponse}, 
    Router,
    routing::{get, post},
};
use std::net::SocketAddr;
use hmac::{Hmac, Mac, digest::MacError};
use sha2::{Sha256};
use tracing::{debug};
use dotenv;
use std::env;
// use hex::decode;

use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    dotenv::dotenv().ok();

    // build our application with a route
    let app = Router::new()
        .route("/", get(handler))
        .route("/", post(hook_handler))
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

async fn hook_handler(headers: HeaderMap, body: String) -> Response {
    debug!("Received request");

    match headers.get("x-hub-signature-256") {
        Some(v) => {
            let their_sig_hex = v.to_str().unwrap_or_default().split("=").collect::<Vec<&str>>().last().copied().unwrap_or_default();
            let their_sig = hex::decode(their_sig_hex).expect("decoding failed");
            match env::var("GITHUB_WEBHOOK_SECRET") {
                Ok(key) => {
                    let mut mac = Hmac::<Sha256>::new_from_slice(key.as_bytes()).expect("HMAC accepts any key size");
                    mac.update(body.as_bytes());
                    match mac.verify_slice(&their_sig) {
                        // Signature verification failed
                        Err(_) => return (StatusCode::UNAUTHORIZED, "unauthorized").into_response(),
                        _ => {
                            debug!("Signature verified")
                            // Signature verified: Process request in this block
                        }
                    }
                },
                // Expected GITHUB_WEBHOOK_SECRET env variable not found
                Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "error: no gh key found").into_response()
            }
        },
       // Expected signature header not found 
        _ => return (StatusCode::UNAUTHORIZED, "unauthorized").into_response()
    }

    (StatusCode::OK, "success").into_response()
}

