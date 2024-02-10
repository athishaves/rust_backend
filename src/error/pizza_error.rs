use actix_web::{
  http::{header::ContentType, StatusCode},
  HttpResponse, ResponseError,
};

use derive_more::Display;

#[derive(Debug, Display)]
pub enum PizzaError {
  NoPizzasFound,
  PizzaCreationFailure,
  InvalidUserInput,
  NoSuchPizzaFound,
  DatabaseFailure,
}

impl ResponseError for PizzaError {
  fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
    HttpResponse::build(self.status_code())
      .insert_header(ContentType::json())
      .body(self.to_string())
  }

  fn status_code(&self) -> StatusCode {
    match self {
      PizzaError::NoPizzasFound => StatusCode::NOT_FOUND,
      PizzaError::PizzaCreationFailure => StatusCode::INTERNAL_SERVER_ERROR,
      PizzaError::InvalidUserInput => StatusCode::BAD_REQUEST,
      PizzaError::NoSuchPizzaFound => StatusCode::NOT_FOUND,
      PizzaError::DatabaseFailure => StatusCode::INTERNAL_SERVER_ERROR,
    }
  }
}
