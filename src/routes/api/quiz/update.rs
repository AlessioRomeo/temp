
use actix_web::{web, HttpRequest, HttpResponse, Result as ActixResult};
use mongodb::bson::{doc, oid::ObjectId, DateTime as MongoDateTime};
use serde::Deserialize;
use crate::config::AppState;
use crate::routes::get_user_from_token;

#[derive(Debug, Deserialize)]
pub struct UpdateQuizData {
    pub title: Option<String>,
    pub description: Option<String>,
    /// if provided, overwrite entire question set
    pub questions: Option<Vec<crate::types::Question>>,
}

pub async fn update_quiz(
    data: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<String>,
    json: web::Json<UpdateQuizData>,
) -> ActixResult<HttpResponse> {
    let user = get_user_from_token(&data, &req).await?;
    let quiz_id = ObjectId::parse_str(&path.into_inner())
        .map_err(|_| actix_web::error::ErrorBadRequest("Invalid quiz id"))?;

    let quiz = data.quiz_col
        .find_one(doc!{ "_id": &quiz_id })
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("DB error"))?
        .ok_or_else(|| actix_web::error::ErrorNotFound("Quiz not found"))?;

    let allowed = quiz.owner_id == user.id
        || quiz.shared_with.iter().any(|sw| sw.user_id == user.id && sw.can_update);
    if !allowed {
        return Err(actix_web::error::ErrorForbidden("No update permission"));
    }

    let mut set = doc! { "updated_at": MongoDateTime::now() };
    if let Some(ref t) = json.title { set.insert("title", t); }
    if let Some(ref d) = json.description { set.insert("description", d); }
    if let Some(ref qs) = json.questions { set.insert("questions", bson::to_bson(qs).unwrap()); }

    data.quiz_col
        .update_one(doc!{ "_id": &quiz_id }, doc!{ "$set": set })
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    let updated = data.quiz_col
        .find_one(doc!{ "_id": &quiz_id })
        .await
        .unwrap()
        .unwrap();
    Ok(HttpResponse::Ok().json(updated))
}
