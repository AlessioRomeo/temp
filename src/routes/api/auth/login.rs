use actix_web::{web, HttpResponse, Result as ActixResult};
use actix_web::web::Data;
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::options::ReplaceOptions;
use serde::Deserialize;
use uuid::Uuid;
use crate::{AppState, Session};
use crate::routes::api::auth::AuthResponse;

/// POST /api/login
/// Tested and also works beautifully
#[derive(Debug, Deserialize)]
pub struct LoginData { email: String, password: String }
pub async fn login(data: Data<AppState>, json: web::Json<LoginData>) -> ActixResult<HttpResponse> {
    let user = data.user_col.find_one(doc! { "email": &json.email })
        .await
        .map_err(|_| actix_web::error::ErrorUnauthorized("Login failed"))?
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Invalid credentials"))?;
    let parsed_hash = PasswordHash::new(&user.password_hash)
        .map_err(|_| actix_web::error::ErrorUnauthorized("Hash parse failed"))?;
    Argon2::default()
        .verify_password(json.password.as_bytes(), &parsed_hash)
        .map_err(|_| actix_web::error::ErrorUnauthorized("Invalid credentials"))?;
    let token = Uuid::new_v4().to_string();
    let session_doc = Session { id: ObjectId::new(), session_id: token.clone(), user_id: user.id.clone() };
    data.session_col
        .insert_one( &session_doc)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;
    Ok(HttpResponse::Ok().json(AuthResponse { token }))
}