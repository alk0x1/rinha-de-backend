use actix_web::{get, post, web, HttpResponse, Responder};
use deadpool_postgres::Pool;
use crate::services;
use crate::model::{CreateTransactionDTO, Extrato};

#[get("/")]
async fn hello() -> impl Responder {
  HttpResponse::Ok().body("Hello world!")
}

#[post("/clientes/{id}/transacoes")]
pub async fn create_transaction(pool: web::Data<Pool>, payload: web::Json<CreateTransactionDTO>) -> Result<HttpResponse, Box<dyn std::error::Error>> {
  
  let transaction = CreateTransactionDTO {
    client_id: payload.client_id,
    valor: payload.valor,
    descricao: payload.descricao.clone(),
    tipo: payload.tipo,
  };

  if payload.tipo == 'd' {
    let saldo = services::get_saldo(&pool.get().await?, payload.client_id).await?;
    if saldo.total - transaction.valor >= saldo.limite {
      services::insert_transaction(&pool.get().await?, transaction).await?;
    }
  }


  Ok(HttpResponse::Created().finish())
}

#[get("/clientes/{id}/extrato")]
pub async fn get_extrato(pool: web::Data<Pool>, path: web::Path<(i32,)>) -> Result<HttpResponse, Box<dyn std::error::Error>> {
  let client_id = path.into_inner().0;
  let saldo = services::get_saldo(&pool.get().await.expect("error on getting the pool"), client_id).await.expect("error on getting the balance");
  let last_transactions = services::get_last_transactions(&pool.get().await.expect("error on getting pool"), client_id).await.expect("Failed to get last transactions");
  
  let extrato = Extrato {
    saldo,
    last_transactions
  };

  let json = serde_json::to_string(&extrato)?;

  Ok(HttpResponse::Ok().body(json))
}
