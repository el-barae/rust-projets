use axum::{routing::get, Router, Json};
use serde::Serialize;
use tokio::net::TcpListener;

async fn root() -> &'static str {
    "OK"
}

#[derive(Serialize)]
struct Info {
    message: &'static str,
    number: u32,
}

async fn json() -> Json<Info> {
    Json(Info {
        message: "Hello from Axum",
        number: 42,
    })
}

async fn compute() -> &'static str {
    let mut sum = 0;
    for i in 0..20000 {
        sum += i;
    }
    "done"
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/json", get(|| async { json().await }))
        .route("/compute", get(|| async { compute().await }));

    println!("Axum running on :8000");

    let listener = TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
