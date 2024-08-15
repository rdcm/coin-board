use crate::domain::{GetRatesQuery, GetRatesQueryHandler};
use crate::http_contracts::{ErrorResponse, RatesResponse};
use actix_web::web::Data;
use actix_web::{get, HttpResponse};

#[get("/rates")]
pub async fn get_rates(handler: Data<dyn GetRatesQueryHandler>) -> HttpResponse {
    let query = GetRatesQuery;

    let option = handler.handle(&query).await;

    match option {
        Some(rates) => HttpResponse::Ok().json(RatesResponse::from_currency_rates(rates)),
        None => HttpResponse::BadRequest().json(ErrorResponse { code: 101 }),
    }
}
