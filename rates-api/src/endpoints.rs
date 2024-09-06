use crate::domain::{GetRatesQuery, GetRatesQueryHandler};
use crate::http_contracts::{ErrorResponse, RatesResponse};
use actix_web::web::Data;
use actix_web::{get, HttpResponse};

#[get("/rates")]
pub async fn get_rates(handler: Data<dyn GetRatesQueryHandler>) -> HttpResponse {
    let query = GetRatesQuery;

    let result = handler.handle(&query).await;

    match result {
        Ok(rates) => HttpResponse::Ok().json(RatesResponse::from_currency_rates(rates)),
        Err(_) => HttpResponse::BadRequest().json(ErrorResponse { code: 101 }),
    }
}
