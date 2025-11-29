use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use serde::Serialize;

#[get("/")]
async fn root() -> impl Responder {
    "Hello from Actix"
}

#[derive(Serialize)]
struct Info {
    message: &'static str,
    number: u32,
}

#[get("/json")]
async fn json() -> impl Responder {
    HttpResponse::Ok().json(Info {
        message: "Hello from Actix",
        number: 42,
    })
}

#[get("/compute")]
async fn compute() -> impl Responder {
    let mut sum = 0;
    for i in 0..20000 {
        sum += i;
    }
    "done"
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    println!("Actix running on :9000");

    HttpServer::new(|| {
        App::new()
            .service(root)
            .service(json)
            .service(compute)
    })
    .bind("0.0.0.0:9000")?
    .run()
    .await
}
