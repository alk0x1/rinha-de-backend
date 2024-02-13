use actix_web::{web, App,  HttpServer};
use deadpool_postgres::{Config, Pool, Runtime};
use tokio_postgres::NoTls;
use std::env;
mod controllers;
mod model;
mod services;


#[tokio::main]
async fn main() -> std::io::Result<()> {
  let mut cfg = Config::new();
  cfg.host = Some(env::var("DB_HOST").unwrap_or_else(|_| "localhost".to_string()));
  cfg.user = Some(env::var("DB_USER").unwrap_or("postgres".to_string()));
  cfg.password = Some(env::var("DB_PASS").unwrap_or("rustwarpcrud".to_string()));
  cfg.dbname = Some(env::var("DB_NAME").unwrap_or("rinha".to_string()));


  let pool: Pool = cfg.create_pool(Some(Runtime::Tokio1), NoTls).unwrap();

  HttpServer::new(move || {
    App::new()
      .app_data(web::Data::new(pool.clone()))
      .service(controllers::hello)
      .service(controllers::create_transaction)
      .service(controllers::get_extrato)
  })
  .bind(("127.0.0.1", 8080))?
  .run()
  .await
}
