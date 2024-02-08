use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
  HttpResponse::Ok().body("Hello world!")
}

#[post("/clientes/{id}/transacoes")]
async fn create_transaction() -> impl Responder {
  HttpResponse::Ok().body("Hello world!")
}