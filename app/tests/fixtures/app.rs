use app::{
    common::{Config, DatabaseDriver},
    util::{Clock, IdGenerator},
    AppBuilder,
};
use axum_test_helper::TestClient;
use ulid::Ulid;

pub static DATETIME_STRING: &str = "2023-11-04T15:32:34.205052Z";

pub struct Dependencies<C: Clock + Clone, G: IdGenerator<Ulid> + Clone> {
    pub clock: C,
    id_generator: G,
}

impl<C: Clock + Clone, G: IdGenerator<Ulid> + Clone> Dependencies<C, G> {
    pub fn new(clock: C, id_generator: G) -> Self {
        Self {
            clock,
            id_generator,
        }
    }
}

pub async fn get_app<C, G>(
    Dependencies {
        clock,
        id_generator,
    }: Dependencies<C, G>,
) -> TestClient
where
    C: Clock + Clone,
    G: IdGenerator<Ulid> + Clone,
{
    let config = Config::new();

    let database_driver = DatabaseDriver::init(&config).await.unwrap();

    let app = AppBuilder::new()
        .clock(clock)
        .config(config)
        .database_driver(database_driver)
        .id_generator(id_generator)
        .build()
        .await
        .unwrap();

    TestClient::new(app)
}
