use std::env;
use mongodb::Client;

pub struct MongoConfig {
    pub uri: String,
    pub db_name: String,
}

impl MongoConfig {
    pub fn from_env() -> Self {
        let uri = env::var("MONGO_URI").expect("MONGO_URI not found");
        let db_name = env::var("DB_NAME").expect("DB_NAME must be set");
        MongoConfig { uri, db_name }
    }

    pub async fn init_client(&self) -> Client {
        Client::with_uri_str(&self.uri)
            .await
            .expect("Failed to connect to MongoDB")
    }
}
