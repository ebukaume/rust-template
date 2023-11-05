use app::{
    common::{Config, Constant, DatabaseDriver},
    util::{SystemClock, SystemTelemetry, UlidGenerator},
    AppBuilder,
};

#[tokio::main]
async fn main() {
    let config = Config::new();
    let _constant = Constant::new();
    let clock = SystemClock::new();
    let telemetry = SystemTelemetry::init(&config);
    let id_generator = UlidGenerator::new();

    let database_driver = DatabaseDriver::init(&config)
        .await
        .expect("Unable to connect to DB!");

    let app = AppBuilder::new()
        .config(config.clone())
        .clock(clock)
        .database_driver(database_driver)
        .telemetry(telemetry)
        .id_generator(id_generator)
        .build()
        .await
        .unwrap_or_else(|_| panic!("Unable to build App"));

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], config.port));

    tracing::info!("Listening on {}", &addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap_or_else(|_| panic!("Unable to start server on {}", &addr))
}
