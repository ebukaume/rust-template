use app::docs::v1::health::HealthStatusResponse;
use axum::http::StatusCode;
use ulid::Ulid;

use crate::fixtures::{
    app::{get_app, Dependencies, DATETIME_STRING},
    clock::MockClock,
    id_generator::MockUlidGenerator,
};

mod fixtures;

#[tokio::test]
async fn returns_200() {
    let id = Ulid::new();
    let clock = MockClock::with_frozen_time(DATETIME_STRING.into());
    let id_generator = MockUlidGenerator::with_fixed_value(&id.to_string());

    let dependencies = Dependencies::new(clock.clone(), id_generator);

    let app = get_app(dependencies).await;

    let res = app.get("/v1/health").send().await;

    assert_eq!(res.status(), StatusCode::OK);
}

#[tokio::test]
async fn returns_health_data() {
    let id = Ulid::new();
    let clock = MockClock::with_frozen_time(DATETIME_STRING.into());
    let id_generator = MockUlidGenerator::with_fixed_value(&id.to_string());

    let dependencies = Dependencies::new(clock.clone(), id_generator);

    let app = get_app(dependencies).await;

    let res = app.get("/v1/health").send().await;
    let body: HealthStatusResponse = res.json().await;

    assert_eq!(
        body,
        HealthStatusResponse {
            api: "OK".to_string(),
            database: "OK".to_string()
        }
    );
}
