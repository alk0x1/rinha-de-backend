use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Transaction {
  pub client_id: i32,
  pub valor: i32,
  pub tipo: char,
  pub descricao: String,
  pub realizada_em: String
}

#[derive(Serialize, Deserialize)]
pub struct Saldo {
  pub total: i32,
  pub data_extrato: String,
  pub limite: i32,
}

#[derive(Serialize, Deserialize)]
pub struct Extrato {
  pub saldo: Saldo,
  pub last_transactions: Vec<Transaction>,
}
