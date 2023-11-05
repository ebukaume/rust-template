use surrealdb::{
    engine::remote::ws::{Client, Ws, Wss},
    opt::auth::Root,
    Surreal,
};
use tracing::info;

use crate::config::Environment;

use super::Config;

pub struct DatabaseDriver {
    pub client: Surreal<Client>,
}

impl DatabaseDriver {
    pub async fn init(config: &Config) -> Result<Self, ()> {
        let client = match config.env {
            Environment::Production => Surreal::new::<Wss>(&config.db_url)
                .await
                .expect("Unable to connect to DB!"),
            _ => Surreal::new::<Ws>(&config.db_url)
                .await
                .expect("Unable to connect to test DB!"),
        };

        info!("Connected to the Database on {}", &config.db_url);

        client
            .signin(Root {
                username: &config.db_username,
                password: &config.db_password,
            })
            .await
            .expect("Failed to authorize DB access!");

        info!("Database access granted to {}", &config.db_username);

        client
            .use_ns(&config.db_namespace)
            .use_db(&config.db_name)
            .await
            .expect("Unable to config namespace!");

        info!(
            "Using {} namespace and {} database",
            &config.db_namespace, &config.db_name
        );

        Ok(Self { client })
    }
}
