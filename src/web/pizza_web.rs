use actix_web::web::{Data, Json, Path};
use actix_web::{get, patch, post};
use uuid::Uuid;
use validator::Validate;

use crate::db::Database;
use crate::domain::{pizza_domain::PizzaDomainTrait, DomainService};
use crate::error::PizzaError;
use crate::models::{BuyPizzaRequest, Pizza, UpdatePizzaUrl};

#[get("/pizzas")]
async fn get_pizzas(db: Data<Database>) -> Result<Json<Vec<Pizza>>, PizzaError> {
  let result = DomainService::get_pizzas(db).await;
  match result {
    Ok(pizzas) => Ok(Json(pizzas)),
    Err(error) => Err(error),
  }
}

#[post("/buypizza")]
async fn buy_pizza(
  body: Json<BuyPizzaRequest>,
  db: Data<Database>,
) -> Result<Json<Pizza>, PizzaError> {
  let is_valid = body.validate();
  match is_valid {
    Ok(_) => {
      let pizza_name = body.pizza_name.clone();

      let mut buffer = Uuid::encode_buffer();
      let new_uuid = Uuid::new_v4().simple().encode_lower(&mut buffer);

      let result =
        DomainService::buy_pizza(db, Pizza::new(String::from(new_uuid), pizza_name)).await;
      match result {
        Ok(pizza) => Ok(Json(pizza)),
        Err(error) => Err(error),
      }
    }
    Err(_) => Err(PizzaError::InvalidUserInput),
  }
}

#[patch("/updatepizza/{uuid}")]
async fn update_pizza(
  update_pizza_url: Path<UpdatePizzaUrl>,
  db: Data<Database>,
) -> Result<Json<Pizza>, PizzaError> {
  let uuid = update_pizza_url.into_inner().uuid;

  let result = DomainService::update_pizza(db, uuid).await;
  match result {
    Ok(pizza) => Ok(Json(pizza)),
    Err(error) => Err(error),
  }
}
