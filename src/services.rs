use crate::dto::Transaction;
use tokio_postgres::Error;
use actix_web::HttpResponse;

pub async fn insert_transaction(conn: &deadpool_postgres::Client, transaction: Transaction) -> Result<HttpResponse, Error> {
  // let query = format!("INSERT INTO rinha (id, client_id, valor, tipo, descricao) VALUES ({}, {}, {}, {}, {})", 
  // transaction.id, transaction.client_id, transaction.valor, transaction.tipo, transaction.descricao);

  let stmt = conn.prepare("INSERT INTO transactions (id, client_id, valor, tipo, descricao) VALUES ($1, $2, $3, $4, $5)").await?;

  conn.execute(&stmt, &[
    &transaction.id, 
    &transaction.client_id, 
    &transaction.valor, 
    &transaction.tipo.to_string(), 
    &transaction.descricao
  ]).await?;

  Ok(HttpResponse::Ok().body("Transaction inserted successfully"))
}