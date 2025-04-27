use actix_web::{web, HttpResponse, Result as ActixResult};
use actix_web::web::Data;
use argon2::{Argon2, PasswordHasher};
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use mongodb::bson::oid::ObjectId;
use crate::{AppState};
use crate::types::User;
use mongodb::error::{Error as MongoError, ErrorKind, WriteFailure};
use serde::Deserialize;

/// POST /api/signup
///Tested and works beautifully
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignupData {
    pub username: String,
    pub email:    String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub profile_picture_url: Option<String>,
}

fn is_duplicate_key_error(e: &MongoError) -> bool {
    match e.kind.as_ref() {
        &ErrorKind::Write(WriteFailure::WriteError(ref write_error))
        if write_error.code == 11000 =>
            {
                true
            }
        _ => false,
    }
}

pub async fn signup(data: Data<AppState>, json: web::Json<SignupData>) -> ActixResult<HttpResponse> {
    let salt = SaltString::generate(&mut OsRng);
    let hash = Argon2::default()
        .hash_password(json.password.as_bytes(), &salt)
        .map_err(|_| actix_web::error::ErrorInternalServerError("Hash error"))?
        .to_string();
    let new_user = User {
        id: ObjectId::new(),
        username: json.username.clone(),
        first_name: json.first_name.clone(),
        last_name: json.last_name.clone(),
        profile_picture_url: json.profile_picture_url.clone(),
        email: json.email.clone(),
        password_hash: hash,
    };
    match data.user_col.insert_one(new_user).await {
        Ok(_) => Ok(HttpResponse::Created().finish()),
        Err(e) if is_duplicate_key_error(&e) => {
            Ok(HttpResponse::Conflict().body("Username or email already exists"))
        }
        Err(e) => {
            log::error!("Failed to insert new user: {}", e);
            Ok(HttpResponse::InternalServerError().finish())
        }
    }
}