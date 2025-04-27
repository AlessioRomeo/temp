use actix_web::{web, HttpRequest, Result as ActixResult, HttpResponse};
use crate::config::AppState;
use crate::routes::get_user_from_token;
use crate::types::Board;
use mongodb::bson::{DateTime as MongoDateTime};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CreateBoardData {
    pub title: String,
    pub description: Option<String>,
}
pub async fn create_board(
    data: web::Data<AppState>,
    req: HttpRequest,
    json: web::Json<CreateBoardData>
) -> ActixResult<HttpResponse> {
    let user = get_user_from_token(&data, &req).await?;
    let now = MongoDateTime::now();
    let board = Board {
        id: ObjectId::new(),
        owner_id: user.id.clone(),
        title: json.title.clone(),
        description: json.description.clone(),
        created_at: now.into(),
        updated_at: now.into(),
        canvas_operations: Vec::new(),
        shared_with: Vec::new(),
        is_owner: None,
    };
    data.board_col
        .insert_one(&board)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;
    Ok(HttpResponse::Created().json(&board))
}