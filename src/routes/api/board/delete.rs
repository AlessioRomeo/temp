use actix_web::{web, HttpRequest, Result as ActixResult, HttpResponse};
use bson::doc;
use crate::config::AppState;
use crate::routes::get_user_from_token;
use mongodb::bson::oid::ObjectId;
pub async fn delete_board(
    data: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<String>,
) -> ActixResult<HttpResponse> {
    let user = get_user_from_token(&data, &req).await?;
    
    let board_id = ObjectId::parse_str(&path.into_inner())
        .map_err(|_| actix_web::error::ErrorBadRequest("Invalid board id"))?;
    let board = data.board_col
        .find_one(doc! { "_id": &board_id })
        .await
        .map_err(|_| actix_web::error::ErrorNotFound("Invalid board id"))?
        .ok_or_else(|| actix_web::error::ErrorNotFound("Board not found"))?;
    
    if(user.id != board.owner_id) {
        return Err(actix_web::error::ErrorForbidden("You are not the owner of this board"));
    }
    
    data.board_col
        .delete_one(doc! {"_id" : &board_id})
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;
    Ok(HttpResponse::Ok().finish())
}