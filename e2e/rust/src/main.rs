//! Generated test application
//! This is a minimal Axum app that echoes back validated parameters

use std::net::SocketAddr;

pub use spikard_e2e_app::*;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = create_app();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
