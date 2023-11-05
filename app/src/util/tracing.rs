use tower_http::{
    classify::{ServerErrorsAsFailures, SharedClassifier},
    trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer},
};
use tracing::Level;

use crate::common::{Config, Environment};

pub struct SystemTelemetry;
pub type Telemetry = TraceLayer<SharedClassifier<ServerErrorsAsFailures>>;

impl SystemTelemetry {
    pub fn init(config: &Config) -> Telemetry {
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
