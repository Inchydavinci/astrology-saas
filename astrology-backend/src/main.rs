use axum::{routing::get, Router};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // Define routes
    let app = Router::new().route("/", get(root_handler));

    // Start the server
    let addr = "0.0.0.0:3000".parse().unwrap();
    println!("🚀 Server running on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root_handler() -> &'static str {
    "Welcome to Astrology SaaS!"
}
