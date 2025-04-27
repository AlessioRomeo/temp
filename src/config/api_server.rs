use std::env;

pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

impl ServerConfig {
    pub fn from_env() -> Self {
        let host = env::var("SERVER_HOST").expect("SERVER_HOST not set");
        let port = env::var("SERVER_PORT")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(8080);
        ServerConfig { host, port }
    }
}
