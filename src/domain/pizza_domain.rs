use actix_web::web::Data;
use async_trait::async_trait;

use crate::db::{pizza_data::PizzaDataTrait, Database};
use crate::domain::DomainService;
use crate::error::PizzaError;
use crate::models::Pizza;

#[async_trait]
pub trait PizzaDomainTrait {
  async fn get_pizzas(db: Data<Database>) -> Result<Vec<Pizza>, PizzaError>;
  async fn buy_pizza(db: Data<Database>, pizza: Pizza) -> Result<Pizza, PizzaError>;
  async fn update_pizza(db: Data<Database>, uuid: String) -> Result<Pizza, PizzaError>;
}

#[async_trait]
impl PizzaDomainTrait for DomainService {
  async fn get_pizzas(db: Data<Database>) -> Result<Vec<Pizza>, PizzaError> {
    let pizzas = Database::get_all_pizzas(&db).await;
    match pizzas {
      Some(found_pizzas) => Ok(found_pizzas),
      None => Err(PizzaError::NoPizzasFound),
    }
  }

  async fn buy_pizza(db: Data<Database>, pizza: Pizza) -> Result<Pizza, PizzaError> {
    let new_pizza = Database::add_pizza(&db, pizza).await;

    match new_pizza {
      Some(created) => Ok(created),
      None => Err(PizzaError::PizzaCreationFailure),
    }
  }

  async fn update_pizza(db: Data<Database>, uuid: String) -> Result<Pizza, PizzaError> {
    let found = Database::get_pizza(&db, uuid.clone()).await;

    match found {
      Some(_) => {
        let updated_pizza = Database::update_pizza(&db, uuid).await;
        match updated_pizza {
          Some(pizza) => Ok(pizza),
          None => Err(PizzaError::DatabaseFailure),
        }
      }
      None => Err(PizzaError::NoSuchPizzaFound),
    }
  }
}
