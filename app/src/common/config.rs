use std::env;

#[derive(PartialEq, Debug, Clone)]
pub enum Environment {
    Production,
    Development,
    Test,
}

#[derive(Debug, Clone)]
pub struct Application {
    pub name: String,
    pub version: String,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub env: Environment,
    pub port: u16,
    pub db_url: String,
    pub db_namespace: String,
    pub db_name: String,
    pub db_username: String,
    pub db_password: String,

    pub app: Application,
}

static MESSAGE_PREFIX: &str = "is required env variable!";

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

impl Config {
    pub fn new() -> Self {
        dotenvy::dotenv().ok();
        let app = Application {
            name: env::var("APP_NAME").unwrap_or_else(|_| panic!("{}", error_message("APP_NAME"))),
            version: env::var("APP_VERSION")
                .unwrap_or_else(|_| panic!("{}", error_message("APP_VERSION"))),
        };
        Self {
            port: env::var("PORT")
                .unwrap_or("4242".into())
                .parse()
                .expect("PORT must be a number"),
            env: env::var("ENV")
                .unwrap_or_else(|_| panic!("{}", error_message("ENV")))
                .to_lowercase()
                .as_str()
                .into(),
            db_url: env::var("SURREALDB_URL")
                .unwrap_or_else(|_| panic!("{}", error_message("SURREALDB_URL"))),
            db_namespace: env::var("SURREALDB_NAMESPACE")
                .unwrap_or_else(|_| panic!("{}", error_message("SURREALDB_NAMESPACE"))),
            db_name: env::var("SURREALDB_DATABASE")
                .unwrap_or_else(|_| panic!("{}", error_message("SURREALDB_DATABASE"))),
            db_username: env::var("SURREALDB_USERNAME")
                .unwrap_or_else(|_| panic!("{}", error_message("SURREALDB_USERNAME"))),
            db_password: env::var("SURREALDB_PASSWORD")
                .unwrap_or_else(|_| panic!("{}", error_message("SURREALDB_PASSWORD"))),
            app,
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
