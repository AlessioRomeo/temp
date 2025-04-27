use actix_web::{web, HttpRequest, HttpResponse, Result as ActixResult};
use mongodb::bson::{doc, oid::ObjectId};
use futures::stream::TryStreamExt; // for try_collect
use chrono::Utc;
use crate::config::AppState;
use crate::routes::get_user_from_token;
use crate::types::Board;
// if you wish to re-serialize dates

/// GET /api/boards
/// List all boards you own or that are shared with you (read or update)
/// You are going to get an empty array on canvas operations for all the boards, this is because we
/// don't want the "list" method to be heavy. Fetch canvas operations when getting a specific board.
pub async fn list_boards(
    data: web::Data<AppState>,
    req: HttpRequest,
) -> ActixResult<HttpResponse> {
    // 1) Authenticate
    let user = get_user_from_token(&data, &req).await?;
    let uid = user.id;

    // 2) Build filter: owner OR shared_with.user_id == you
    let filter = doc! {
        "$or": [
            { "owner_id": &uid },
            { "shared_with.user_id": &uid }
        ]
    };

    // 3) Execute query, excluding the heavy `canvas_operations` if you like:
    let mut cursor = data.board_col
        .find(
            filter,
        )
        .projection(doc! {"canvas_operations" : 0})
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    // 4) Collect into a Vec<Board>
    let boards: Vec<Board> = cursor
        .try_collect()
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    // 5) Return JSON array
    Ok(HttpResponse::Ok().json(boards))
}
