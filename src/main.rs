// Minimal Perch demo app: binds $BIND (Perch sets it to the tailnet IP:port).
use axum::{Router, routing::get};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route(
            "/",
            get(|| async { "hello from perch-hello 🐦 - deployed by Perch\nHi2\n" }),
        )
        .route("/healthz", get(|| async { "ok" }));
    let addr = std::env::var("BIND").unwrap_or_else(|_| "0.0.0.0:8080".into());
    println!("perch-hello listening on {addr}");
    let l = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(l, app).await.unwrap();
}
