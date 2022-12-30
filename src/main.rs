// Run with `RUST_LOG=debug cargo run`

use axum::{
    // body::Bytes,
    http::{HeaderMap, StatusCode},
    response::{Html, Response, IntoResponse}, 
    Router,
    routing::{get, post},
};
use once_cell::sync::Lazy;
use std::net::SocketAddr;
use hmac::{Hmac, Mac};
use sha2::{Sha256};
use tracing::{debug};
use dotenv;
use std::env;
// use hex::decode;

use tower_http::trace::TraceLayer;

static GITHUB_WEBHOOK_SECRET: Lazy<String> =
    Lazy::new(|| env::var("GITHUB_WEBHOOK_SECRET").unwrap());

// get custom header
fn get_custom_header<'a>(headers: &'a HeaderMap, key: &'a str) -> Option<&'a str> {
    let header = headers.get(key).and_then(|header| header.to_str().ok());
    header?.split("=").collect::<Vec<&str>>().last().copied()
}

// validate signature against secret
fn validate_signature(secret: String, signature: &str, body: String) -> Result<bool, String> {
    let sig_decoded = match hex::decode(signature) {
        Ok(v) => v,
        Err(e) => return Err(e.to_string()),
    };

    let mut mac = match Hmac::<Sha256>::new_from_slice(secret.as_bytes()) {
        Ok(v) => v,
        Err(e) => return Err(e.to_string()),
    };
    mac.update(body.as_bytes());
    if let Err(e) = mac.verify_slice(&sig_decoded) {
        return Err(e.to_string());
    };

    Ok(true)
}


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

    let sig_value = if let Some(sig_value) = get_custom_header(&headers, "x-hub-signature-256") {
        sig_value
    } else {
        return (StatusCode::UNAUTHORIZED, "unauthorized").into_response();
    };

    if let Err(e) = validate_signature(GITHUB_WEBHOOK_SECRET.to_string(), &sig_value, body) {
        debug!(err = e.to_string());
        return (StatusCode::UNAUTHORIZED, "unauthorized").into_response();
    };
    debug!("Signature verified");

    (StatusCode::OK, "success").into_response()
}

