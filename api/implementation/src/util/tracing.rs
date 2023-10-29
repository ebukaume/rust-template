use tower_http::{
    classify::{ServerErrorsAsFailures, SharedClassifier},
    trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer},
};
use tracing::Level;

use crate::{common::Environment, Config};

pub struct Tracing;

impl Tracing {
    pub fn init(config: &Config) -> TraceLayer<SharedClassifier<ServerErrorsAsFailures>> {
        tracing_subscriber::fmt()
            .compact()
            .with_target(false)
            .init();

        match config.env {
            Environment::Production => TraceLayer::new_for_http().make_span_with(
                DefaultMakeSpan::new()
                    .level(Level::INFO)
                    .include_headers(false),
            ),
            _ => TraceLayer::new_for_http()
                .make_span_with(
                    DefaultMakeSpan::new()
                        .level(Level::INFO)
                        .include_headers(false),
                )
                .on_response(
                    DefaultOnResponse::new()
                        .level(Level::INFO)
                        .latency_unit(tower_http::LatencyUnit::Millis),
                )
                .on_request(DefaultOnRequest::new().level(Level::INFO)),
        }
    }
}
