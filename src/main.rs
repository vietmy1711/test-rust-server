use actix_web::{post, App, HttpResponse, HttpServer, Responder};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(commit_received))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

#[post("/commit_received")]
async fn commit_received(commit_hash: String) -> impl Responder {
    HttpResponse::Ok().body(commit_hash)
}
