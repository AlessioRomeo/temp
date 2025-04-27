mod login;
mod signup;
mod whoami;
mod logout;

use actix_web::{HttpRequest, Result as ActixResult};
use mongodb::bson::doc;
use serde::Serialize;
pub use login::*;
pub use signup::*;
pub use whoami::*;
pub use logout::*;
use crate::config::AppState;
use crate::types::User;

#[derive(Debug, Serialize)]
struct AuthResponse { pub token: String }
pub async fn get_user_from_token(state: &AppState, req: &HttpRequest) -> ActixResult<User> {


    let token = req.headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|h| h.strip_prefix("Bearer "))
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Missing or invalid token"))?;

    let session = state.session_col
        .find_one(doc! { "session_id": token })
        .await
        .map_err(|_| actix_web::error::ErrorUnauthorized("Session lookup failed"))?
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Session not found"))?;


    let user = state.user_col
        .find_one(doc! { "_id": &session.user_id })
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("User lookup failed"))?
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("User not found"))?;


    Ok(user)
}