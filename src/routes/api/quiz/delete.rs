use actix_web::{web, HttpRequest, HttpResponse, Result as ActixResult};
use mongodb::bson::{doc, oid::ObjectId};
use crate::config::AppState;
use crate::routes::get_user_from_token;

pub async fn delete_quiz(
    data: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<String>,
) -> ActixResult<HttpResponse> {
    let user = get_user_from_token(&data, &req).await?;
    let quiz_id = ObjectId::parse_str(&path.into_inner())
        .map_err(|_| actix_web::error::ErrorBadRequest("Invalid quiz id"))?;

    let quiz = data.quiz_col
        .find_one(doc!{ "_id": &quiz_id })
        .await
        .map_err(|_| actix_web::error::ErrorNotFound("Invalid quiz id"))?
        .ok_or_else(|| actix_web::error::ErrorNotFound("Quiz not found"))?;
    if quiz.owner_id != user.id {
        return Err(actix_web::error::ErrorForbidden("Not the owner"));
    }

    data.quiz_col
        .delete_one(doc!{ "_id": &quiz_id })
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;
    Ok(HttpResponse::Ok().finish())
}
