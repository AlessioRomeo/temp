mod app_state;

pub mod mongo;
pub mod api_server;

pub use mongo::MongoConfig;
pub use api_server::ServerConfig;
pub use app_state::AppState;

pub struct AppConfig {
    pub mongo: MongoConfig,
    pub server: ServerConfig,
}

impl AppConfig {
    pub fn from_env() -> Self {
        AppConfig {
            mongo: MongoConfig::from_env(),
            server: ServerConfig::from_env(),
        }
    }
}
