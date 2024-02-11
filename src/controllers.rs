use actix_web::{get, post, web, HttpResponse, Responder};
use deadpool_postgres::Pool;
use crate::services;
use crate::dto::Transaction;

#[get("/")]
async fn hello() -> impl Responder {
  HttpResponse::Ok().body("Hello world!")
}

#[post("/clientes/{id}/transacoes")]
pub async fn create_transaction(pool: web::Data<Pool>) -> Result<HttpResponse, Box<dyn std::error::Error>> {
  let transaction = Transaction {
    id: 0,
    client_id: 0,
    valor: 0,
    descricao: String::from("teste"),
    tipo: 'c'
  };

  services::insert_transaction(&pool.get().await?, transaction).await?;

  Ok(HttpResponse::Created()
  .finish())}
