use actix_web::{HttpRequest, HttpResponse, Result as ActixResult};
use actix_web::web::Data;
use mongodb::bson::doc;
use crate::{AppState};



/// DELETE /api/logout
/// Tested and also works beautifully
pub async fn logout(data: Data<AppState>, req: HttpRequest,) -> ActixResult<HttpResponse> {
    let token = req.headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|h| h.strip_prefix("Bearer "))
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Missing or invalid token"))?;
    data.session_col
        .delete_one( doc! {"_id" : token})
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;
    Ok(HttpResponse::Ok().finish())
}