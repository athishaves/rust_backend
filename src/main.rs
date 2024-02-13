mod create_app;
mod db;
mod domain;
mod error;
mod models;
mod web;
use std::process::exit;

use crate::create_app::create_app;
use actix_web::{web::Data, HttpServer};
use db::Database;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  let db: Database;
  let result = Database::init().await;
  match result {
    Ok(_db) => db = _db,
    Err(err) => {
      println!("Error -> {err}");
      exit(100)
    }
  }
  let db_data = Data::new(db);

  env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

  HttpServer::new(move || create_app(db_data.clone()))
    .bind("172.18.0.3:8080")?
    .run()
    .await
}
