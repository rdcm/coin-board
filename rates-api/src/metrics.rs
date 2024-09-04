use actix_web::{dev::ServiceRequest, dev::ServiceResponse, web, HttpResponse};
use prometheus::{register_counter, register_histogram, Counter, Encoder, Histogram, TextEncoder};
use std::sync::Arc;

pub struct Metrics {
    pub request_count: Counter,
    pub request_duration: Histogram,
}

impl Metrics {
    pub fn new() -> Self {
        Metrics {
            request_count: register_counter!(
                "request_count",
                "Total number of HTTP requests made."
            )
            .unwrap(),
            request_duration: register_histogram!(
                "request_duration_seconds",
                "The HTTP request latencies in seconds."
            )
            .unwrap(),
        }
    }
}

pub async fn metrics_handler(metrics: web::Data<Arc<Metrics>>) -> HttpResponse {
    let mut buffer = Vec::new();
    let encoder = TextEncoder::new();
    let metric_families = prometheus::gather();
    encoder.encode(&metric_families, &mut buffer).unwrap();
    HttpResponse::Ok()
        .content_type(encoder.format_type())
        .body(buffer)
}
