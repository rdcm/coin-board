#![allow(unused_imports)]
use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse},
    web::Data,
    Error, HttpResponse,
};

use anyhow::{Context, Result};
use prometheus::{register_counter, register_histogram, Counter, Encoder, Histogram, TextEncoder};
use std::sync::Arc;
use tracing::error;

pub struct Metrics {
    pub request_count: Counter,
    pub request_duration: Histogram,
}

impl Metrics {
    pub fn new() -> Result<Self> {
        let request_count =
            register_counter!("request_count", "Total number of HTTP requests made.")
                .context("[rates-api] [prometheus] Failed to register 'request_count' metric")?;

        let request_duration = register_histogram!(
            "request_duration_seconds",
            "The HTTP request latencies in seconds."
        )
        .context("[rates-api] [prometheus] Failed to register 'request_duration_seconds' metric")?;

        Ok(Metrics {
            request_count,
            request_duration,
        })
    }
}

pub async fn get_metrics_handler(_: Data<Arc<Metrics>>) -> Result<HttpResponse, Error> {
    let mut buffer = Vec::new();
    let encoder = TextEncoder::new();

    let metric_families = prometheus::gather();

    encoder.encode(&metric_families, &mut buffer).map_err(|e| {
        error!("[rates-api] [prometheus] Metrics encoding error: {}", e);
        actix_web::error::ErrorInternalServerError(e)
    })?;

    Ok(HttpResponse::Ok()
        .content_type(encoder.format_type())
        .body(buffer))
}
