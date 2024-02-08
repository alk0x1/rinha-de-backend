use actix_web::{web, App,  HttpServer};
mod controllers;

#[tokio::main]
async fn main() -> std::io::Result<()> {
  HttpServer::new(move || {
    App::new()
      .service(controllers::hello)
      .service(controllers::create_transaction)
  })
  .bind(("127.0.0.1", 8080))?
  .run()
  .await
}