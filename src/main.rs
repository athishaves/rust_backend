mod create_app;
mod db;
mod domain;
mod error;
mod models;
mod web;
use crate::create_app::create_app;
use actix_web::{web::Data, HttpServer};
use db::Database;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  let db = Database::init()
    .await
    .expect("error connecting to database");
  let db_data = Data::new(db);

  HttpServer::new(move || create_app(db_data.clone()))
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
