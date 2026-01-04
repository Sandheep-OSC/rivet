use std::env;

#[derive(Clone)]
pub struct Config {
    pub server_port: u16,
    pub database_url: String,
}

impl Config {
    pub fn from_env() -> Self {
        let server_port = env::var("SERVER_PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse()
            .expect("Invalid SERVER_PORT");

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        Self {
            server_port,
            database_url,
        }
    }
}
