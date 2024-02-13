use crate::db::Database;
use crate::web::pizza_web;
use actix_web::body::MessageBody;
use actix_web::dev::{ServiceFactory, ServiceRequest, ServiceResponse};
use actix_web::middleware::Logger;
use actix_web::web::{self, Data};
use actix_web::{App, Error};

pub fn create_app(
  db_data: Data<Database>,
) -> App<
  impl ServiceFactory<
    ServiceRequest,
    Response = ServiceResponse<impl MessageBody>,
    Config = (),
    InitError = (),
    Error = Error,
  >,
> {
  App::new()
    .app_data(db_data)
    .wrap(Logger::default())
    .service(
      web::scope("/v1")
        .service(pizza_web::get_pizzas)
        .service(pizza_web::buy_pizza)
        .service(pizza_web::update_pizza),
    )
}
