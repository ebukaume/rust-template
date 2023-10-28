use std::env;

#[derive(PartialEq, Debug)]
pub enum Environment {
    Production,
    Development,
    Test,
}

#[derive(Debug)]
pub struct Config {
    pub env: Environment,
    pub port: u16,
    pub db_url: String,
    pub db_namespace: String,
    pub db_name: String,
    pub db_username: String,
    pub db_password: String,
}

static MESSAGE_PREFIX: &str = "is required env variable!";

impl Config {
    pub fn new() -> Self {
        dotenv::dotenv().ok();
        Self {
            port: env::var("PORT")
                .unwrap_or("4242".into())
                .parse()
                .expect("PORT must be a number"),
            env: env::var("ENV")
                .expect(&error_message("ENV"))
                .to_lowercase()
                .as_str()
                .into(),
            db_url: env::var("SURREALDB_URL").expect(&error_message("SURREALDB_URL")),
            db_namespace: env::var("SURREALDB_NAMESPACE")
                .expect(&error_message("SURREALDB_NAMESPACE")),
            db_name: env::var("SURREALDB_DATABASE").expect(&error_message("SURREALDB_DATABASE")),
            db_username: env::var("SURREALDB_USERNAME")
                .expect(&error_message("SURREALDB_USERNAME")),
            db_password: env::var("SURREALDB_PASSWORD")
                .expect(&error_message("SURREALDB_PASSWORD")),
        }
    }
}

impl From<&str> for Environment {
    fn from(value: &str) -> Self {
        match value {
            "prod" => Environment::Production,
            "dev" => Environment::Development,
            "test" => Environment::Test,
            _ => panic!("Unknown environment"),
        }
    }
}

fn error_message(prefix: &str) -> String {
    format!("{prefix} {MESSAGE_PREFIX}")
}
