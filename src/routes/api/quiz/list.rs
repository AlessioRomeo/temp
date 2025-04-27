use actix_web::{web, HttpRequest, HttpResponse, Result as ActixResult};
use futures::stream::TryStreamExt;
use mongodb::bson::{doc};
use crate::config::AppState;
use crate::routes::get_user_from_token;
use crate::types::Quiz;

pub async fn list_quizzes(
    data: web::Data<AppState>,
    req: HttpRequest,
) -> ActixResult<HttpResponse> {
    let user = get_user_from_token(&data, &req).await?;
    let uid = user.id;

    let filter = doc! {
        "$or": [
            { "owner_id": &uid },
            { "shared_with.user_id": &uid }
        ]
    };

    let mut cursor = data.quiz_col
        .find(filter)
        .projection(doc!{ "questions": 0 })
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;
        

    let quizzes: Vec<Quiz> = cursor
        .try_collect()
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    Ok(HttpResponse::Ok().json(quizzes))
}
