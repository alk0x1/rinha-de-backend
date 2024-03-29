use crate::model::{CreateTransactionDTO, Saldo, Transaction};
use tokio_postgres::Error;
use actix_web::HttpResponse;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::time::SystemTime;

pub async fn insert_transaction(conn: &deadpool_postgres::Client, transaction: CreateTransactionDTO) -> Result<HttpResponse, Error> {
  let stmt = conn.prepare("INSERT INTO transactions (id, client_id, valor, tipo, descricao) VALUES ($1, $2, $3, $4, $5)").await?;

  let id = Uuid::new_v4().to_string();

  conn.execute(&stmt, &[
    &id, 
    &transaction.client_id, 
    &transaction.valor, 
    &transaction.tipo.to_string(), 
    &transaction.descricao
  ]).await?;

  Ok(HttpResponse::Ok().body("Transaction inserted successfully"))
}

pub async fn get_saldo(conn: &deadpool_postgres::Client, client_id: i32) -> Result<Saldo, Error>  {
  let stmt = conn.prepare("SELECT saldo, limite, created_at FROM clientes WHERE id = $1").await.expect("error in prepare statement");
  let row = conn.query_one(&stmt, &[&client_id]).await.expect("error on requesting the balance");

  let total = row.get("saldo");
  let limite = row.get("limite");
  let created_at: SystemTime = row.get("created_at");
  let data_extrato = DateTime::<Utc>::from(created_at).format("%Y-%m-%dT%H:%M:%S%.6fZ").to_string();

  Ok(Saldo {
    total,
    limite,
    data_extrato,
  })
}

pub async fn get_last_transactions(conn: &deadpool_postgres::Client, client_id: i32) -> Result<Vec<Transaction>, Error> {
  let stmt: tokio_postgres::Statement = conn.prepare("SELECT valor, tipo, descricao, created_at FROM transactions WHERE client_id = $1 ORDER BY created_at DESC LIMIT 10").await?;
  let rows = conn.query(&stmt, &[&client_id]).await.expect("Failed to execute query");
  let mut transactions: Vec<Transaction> = Vec::new();

  for row in rows {
    let valor = row.get("valor");
    let tipo: String = row.get("tipo");
    let tipo_char = tipo.chars().next().expect("tipo is empty");
    
    let descricao = row.get("descricao");

    let created_at: SystemTime = row.get("created_at");
    let realizada_em = DateTime::<Utc>::from(created_at).format("%Y-%m-%dT%H:%M:%S%.6fZ").to_string();

    transactions.push(Transaction {
      client_id,
      valor,
      tipo: tipo_char,
      descricao,
      realizada_em
    });
  }
  Ok(transactions)
}