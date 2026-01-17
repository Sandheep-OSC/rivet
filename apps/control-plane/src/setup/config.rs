use config::{Config as ConfigBuilder, ConfigError};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    pub app: AppConfig,
    pub database: DatabaseConfig,
}

#[derive(Clone, Debug, Deserialize)]
pub struct AppConfig {
    pub host: String,
    pub port: u16,
}

impl AppConfig {
    pub fn addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        let config = ConfigBuilder::builder()
            .set_default("app.host", "127.0.0.1")?
            .set_default("app.port", 8080)?
            .set_default(
                "database.url",
                "postgres://user:password@localhost:5432/rivet",
            )?
            .add_source(
                config::Environment::with_prefix("APP")
                    .separator("_")
                    .try_parsing(true),
            )
            .build()?;

        config.try_deserialize()
    }

    pub fn database_url(&self) -> &str {
        &self.database.url
    }
}
