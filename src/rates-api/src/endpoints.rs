use crate::domain::{GetRatesQuery, GetRatesQueryHandler};
use crate::http_contracts::{ErrorResponse, RatesResponse};
use actix_web::web::Data;
use actix_web::{get, HttpResponse};
const SOURCE: &str = "Binance";

#[get("/rates")]
pub async fn get_rates(handler: Data<dyn GetRatesQueryHandler>) -> HttpResponse {
    let query = GetRatesQuery { source: SOURCE };

    let option = handler.handle(&query).await;

    match option {
        Some(rates) => HttpResponse::Ok().json(RatesResponse::from_currency_rates(
            SOURCE.to_string(),
            rates,
        )),
        None => HttpResponse::BadRequest().json(ErrorResponse { code: 101 }),
    }
}
