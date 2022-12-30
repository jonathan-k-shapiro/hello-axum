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
//  env::var("GITHUB_WEBHOOK_SECRET")?;

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

    // match headers.get("x-hub-signature-256") {
    //     Some(v) => {
    //         // x-hub-signature-256 header has value that looks like "sha256=<a bunch of hex digits>"
    //         // We need to just take the hex digits and convert them into an array of bytes.
    //         let split = v.to_str().unwrap_or_default().split("=");
    //         let their_sig_hex = split.collect::<Vec<_>>().last().copied().unwrap_or_default();
    //         let their_sig = hex::decode(their_sig_hex).expect("decoding failed");
    // match env::var("GITHUB_WEBHOOK_SECRET") {
    //     Ok(key) => {
    //         let mut mac = Hmac::<Sha256>::new_from_slice(key.as_bytes()).expect("HMAC accepts any key size");
    //         mac.update(body.as_bytes());
    //         match mac.verify_slice(&their_sig) {
    //             // Signature verification failed
    //             Err(_) => return (StatusCode::UNAUTHORIZED, "unauthorized").into_response(),
    //             _ => {
    //                 debug!("Signature verified")
    //                 // Signature verified: Process request in this block
    //             }
    //         }
    //     },
    //     // Expected GITHUB_WEBHOOK_SECRET env variable not found
    //     Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "error: no gh key found").into_response()
    // }
// },
// Expected signature header not found 
// _ => return (StatusCode::UNAUTHORIZED, "unauthorized").into_response()
    // }

    // (StatusCode::OK, "success").into_response()
// }

