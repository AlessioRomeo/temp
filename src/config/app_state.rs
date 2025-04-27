use actix_web::web;
use mongodb::{Client, Collection};
use crate::{config, Session};
use crate::types::{Board, Quiz, User};

pub struct AppState {
    pub user_col: Collection<User>,
    pub session_col: Collection<Session>,
    pub quiz_col: Collection<Quiz>,
    pub board_col: Collection<Board>
}

impl AppState {
    /// Initialize AppState before the server starts
    pub async fn init(cfg: &config::MongoConfig) -> web::Data<Self> {
        let client = Client::with_uri_str(&cfg.uri)
            .await
            .expect("Failed to connect to MongoDB");
        let db = client.database(&cfg.db_name);
        let state = AppState {
            user_col: db.collection::<User>("users"),
            session_col: db.collection::<Session>("sessions"),
            quiz_col: db.collection::<Quiz>("quizzes"),
            board_col: db.collection::<Board>("boards"),
        };
        web::Data::new(state)
    }
}
