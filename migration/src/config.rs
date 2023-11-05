use std::env;

#[derive(Debug)]
pub struct Config {
    pub env: Environment,
    pub db_url: String,
    pub db_namespace: String,
    pub db_name: String,
    pub db_username: String,
    pub db_password: String,
}

#[derive(PartialEq, Debug)]
pub enum Environment {
    Production,
    Development,
    Test,
}

static MESSAGE_PREFIX: &str = "is required env variable!";

impl Config {
    pub fn new() -> Self {
        dotenvy::dotenv().ok();
        Self {
            env: env::var("ENV")
                .unwrap_or("dev".to_owned())
                .to_lowercase()
                .as_str()
                .into(),
            db_url: env::var("SURREALDB_URL").unwrap_or_else(|_| { panic!("{}", error_message("SURREALDB_URL")) }),
            db_namespace: env::var("SURREALDB_NAMESPACE")
                .unwrap_or_else(|_| { panic!("{}", error_message("SURREALDB_NAMESPACE")) }),
            db_name: env::var("SURREALDB_DATABASE").unwrap_or_else(|_| { panic!("{}", error_message("SURREALDB_DATABASE")) }),
            db_username: env::var("SURREALDB_USERNAME")
                .unwrap_or_else(|_| { panic!("{}", error_message("SURREALDB_USERNAME")) }),
            db_password: env::var("SURREALDB_PASSWORD")
                .unwrap_or_else(|_| { panic!("{}", error_message("SURREALDB_PASSWORD")) }),
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
