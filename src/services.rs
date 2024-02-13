use crate::model::{Saldo, Transaction};
use tokio_postgres::Error;
use actix_web::HttpResponse;
use uuid::Uuid;

pub async fn insert_transaction(conn: &deadpool_postgres::Client, transaction: Transaction) -> Result<HttpResponse, Error> {
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
  let stmt = conn.prepare("SELECT saldo, limite, created_at FROM clientes WHERE id = $1").await?;
  let row = conn.query_one(&stmt, &[&client_id]).await?;

  let total = row.get("saldo");
  let limite = row.get("limite");
  let data_extrato = row.get("created_at");

  Ok(Saldo {
    total,
    limite,
    data_extrato,
  })
}

pub async fn get_last_transactions(conn: &deadpool_postgres::Client, client_id: i32) -> Result<Vec<Transaction>, Error> {
  let stmt: tokio_postgres::Statement = conn.prepare("SELECT valor, tipo, descricao, created_at FROM transactions WHERE client_id = $1 ORDER BY created_at DESC LIMIT 10").await?;
  let rows = conn.query(&stmt, &[&client_id]).await?;
  let mut transactions: Vec<Transaction> = Vec::new();

  for row in rows {
    let valor = row.get("valor");
    let tipo: String = row.get("tipo");
    let tipo_char = tipo.chars().next().expect("tipo is empty");
    
    let descricao = row.get("descricao");
    let realizada_em = row.get("created_at");

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