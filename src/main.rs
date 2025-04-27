mod routes;
mod types;
mod config;

use actix::Actor;
use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use actix_web::middleware::Logger;
use mongodb::{bson::{doc, oid::ObjectId, DateTime as MongoDateTime}};
use serde::{Serialize, Deserialize};
use routes::{login, signup, whoami};
use crate::config::{AppConfig, AppState};
use dotenv::dotenv;
use crate::routes::{board_ws, create_board, create_quiz, delete_board, delete_quiz, get_board, get_quiz, list_boards, list_quizzes, logout, share_board, share_quiz, update_board, update_quiz};
use crate::types::{BoardServer, SharedWith};

#[derive(Debug, Serialize, Deserialize)]
struct Session {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub session_id: String,
    pub user_id: ObjectId,
}

#[derive(Debug, Serialize, Deserialize)]
struct Note {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub owner_id: ObjectId,
    pub title: String,
    pub description: String,
    pub created_at: MongoDateTime,
    pub updated_at: MongoDateTime,
    pub canvas_operations: Vec<serde_json::Value>,
    pub shared_with: Vec<SharedWith>,
}




#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    // 1) Load config and init AppState
    let cfg   = AppConfig::from_env();
    let state = AppState::init(&cfg.mongo).await;

    // 2) Start the single BoardServer actor
    //    so it lives for the lifetime of the app
    let board_srv = BoardServer::new(state.clone()).start();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(
                Cors::default()
                    .allow_any_header()
                    .allow_any_origin()
                    .allow_any_method()
            )
            // 3) make AppState available to all handlers
            .app_data(state.clone())
            // 4) make the BoardServer address available too
            .app_data(web::Data::new(board_srv.clone()))

            // 5) Mount WS endpoint outside /api so it's
            //    GET /ws/boards/{id}
            .service(
                web::resource("/ws/boards/{id}")
                    .route(web::get().to(board_ws))
            )

            // 6) existing HTTP API under /api
            .service(
                web::scope("/api")
                    .service(
                        web::scope("/auth")
                            .route("/signup", web::post().to(signup))
                            .route("/login",  web::post().to(login))
                            .route("/logout", web::delete().to(logout))
                            .route("/whoami", web::get().to(whoami))
                    )
                    .service(
                        web::scope("/boards")
                            .route("/create",      web::post().to(create_board))
                            .route("/{id}",        web::get().to(get_board))
                            .route("/{id}/update", web::put().to(update_board))
                            .route("/{id}",        web::delete().to(delete_board))
                            .route("/list",        web::get().to(list_boards))
                            .route("/{id}/share",  web::post().to(share_board))
                    )
                    .service(
                        web::scope("/quizzes")
                            .route("/create", web::post().to(create_quiz))
                            .route("/{id}", web::get().to(get_quiz))
                            .route("/{id}/update", web::put().to(update_quiz))
                            .route("/{id}", web::delete().to(delete_quiz))
                            .route("/list", web::get().to(list_quizzes))
                            .route("/{id}/share", web::post().to(share_quiz))
                    )
            )
    })
        .bind((cfg.server.host.as_str(), cfg.server.port))?
        .run()
        .await
}
