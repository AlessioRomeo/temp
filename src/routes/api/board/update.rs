use serde::Deserialize;
use mongodb::bson::{doc, oid::ObjectId, DateTime as MongoDateTime};
use actix_web::{web, HttpRequest, HttpResponse, Result as ActixResult};
use crate::{AppState};
use crate::routes::get_user_from_token;

#[derive(Debug, Deserialize)]
pub struct UpdateBoardData {
    pub title:       Option<String>,
    pub description: Option<String>,
}

pub async fn update_board(
    data: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<String>,
    json: web::Json<UpdateBoardData>,
) -> ActixResult<HttpResponse> {
    // 1) Auth
    let me = get_user_from_token(&data, &req).await?;
    let board_id = ObjectId::parse_str(&path.into_inner())
        .map_err(|_| actix_web::error::ErrorBadRequest("Invalid board id"))?;

    // 2) Load & ACL check
    let board = data.board_col
        .find_one(doc!{ "_id": &board_id })
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("DB error"))?
        .ok_or_else(|| actix_web::error::ErrorNotFound("Board not found"))?;
    let allowed = board.owner_id == me.id
        || board.shared_with.iter().any(|sw| sw.user_id == me.id && sw.can_update);
    if !allowed {
        return Err(actix_web::error::ErrorForbidden("No update permission"));
    }

    // 3) Build $set doc (only title & description + updated_at)
    let mut set_doc = doc! { "updated_at": MongoDateTime::now() };
    if let Some(ref t) = json.title {
        set_doc.insert("title", t);
    }
    if let Some(ref d) = json.description {
        set_doc.insert("description", d);
    }

    // 4) Persist
    data.board_col
        .update_one(doc!{ "_id": &board_id }, doc!{ "$set": set_doc })
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    // 5) Return updated board
    let updated = data.board_col
        .find_one(doc!{ "_id": &board_id })
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("DB error"))?
        .unwrap();
    Ok(HttpResponse::Ok().json(updated))
}
