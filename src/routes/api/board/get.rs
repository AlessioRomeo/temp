use actix_web::{web, HttpRequest, HttpResponse, Result as ActixResult};
use mongodb::bson::{doc, oid::ObjectId};
use crate::config::AppState;
use crate::routes::get_user_from_token;

pub async fn get_board(
    data: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<String>,
) -> ActixResult<HttpResponse> {
    // 1) authenticate
    let user = get_user_from_token(&data, &req).await?;

    // 2) parse & load board
    let board_id = ObjectId::parse_str(&path.into_inner())
        .map_err(|_| actix_web::error::ErrorBadRequest("Invalid board id"))?;
    let board = data.board_col
        .find_one(doc! { "_id": &board_id })
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("DB error"))?
        .ok_or_else(|| actix_web::error::ErrorNotFound("Board not found"))?;

    // 3) check ACL: owner OR shared_with
    let is_owner = board.owner_id == user.id;
    let is_shared = board
        .shared_with
        .iter()
        .any(|sw| sw.user_id == user.id);
    if !is_owner && !is_shared {
        return Err(actix_web::error::ErrorForbidden("Access denied"));
    }

    // 4) return the board
    Ok(HttpResponse::Ok().json(board))
}
