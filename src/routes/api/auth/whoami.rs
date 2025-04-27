use actix_web::{HttpRequest, HttpResponse, Responder, Result as ActixResult};
use actix_web::web::Data;
use mongodb::bson::doc;
use crate::{AppState};
use crate::routes::get_user_from_token;
use crate::types::User;

/// GET /api/whoami
/// Tested and works beautifully


pub async fn whoami(data: Data<AppState>, req: HttpRequest) -> impl Responder {
    match get_user_from_token(&data, &req).await {
        Ok(mut user) => { user.password_hash.clear(); HttpResponse::Ok().json(user) }
        Err(e) => e.error_response(),
    }
}