use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub db_user: String,
    pub db_password: String,
    pub db_name: String,
    pub db_host: String,
    pub db_port: u16,
    pub addr: String,
}

impl Config {
    pub fn from_env() -> Self {
        let db_user = env::var("DB_USER").unwrap_or_else(|_| "postgres".to_string());
        let db_password = env::var("DB_PASSWORD").unwrap_or_else(|_| "password".to_string());
        let db_name = env::var("DB_NAME").unwrap_or_else(|_| "hehe".to_string());
        let db_host = env::var("DB_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        let db_port = env::var("DB_PORT").ok().and_then(|s| s.parse().ok()).unwrap_or(5432);
        let addr = env::var("ADDR").unwrap_or_else(|_| "127.0.0.1:3000".to_string());

        Config {
            db_user,
            db_password,
            db_name,
            db_host,
            db_port,
            addr,
        }
    }

    pub fn database_url(&self) -> String {
        format!("postgres://{}:{}@{}:{}/{}", self.db_user, self.db_password, self.db_host, self.db_port, self.db_name)
    }
}
