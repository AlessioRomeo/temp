use actix_web::{web, HttpRequest, HttpResponse, Result as ActixResult};
use crate::config::AppState;
use crate::routes::get_user_from_token;
use crate::types::{Quiz, Question};
use mongodb::bson::{oid::ObjectId, DateTime as MongoDateTime};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CreateQuizData {
    pub title: String,
    pub description: Option<String>,
    /// if present, AI will generate questions from this image URL
    pub image_url: Option<String>,
}

pub async fn create_quiz(
    data: web::Data<AppState>,
    req: HttpRequest,
    json: web::Json<CreateQuizData>,
) -> ActixResult<HttpResponse> {
    let user = get_user_from_token(&data, &req).await?;
    let now = MongoDateTime::now();
    let mut questions: Vec<Question> = Vec::new();

    //todo: connect AI here
    // if let Some(ref url) = json.image_url {
    //     // stub: replace with your AI client call
    //     questions = data
    //         .ai_client
    //         .generate_quiz_from_image(url)
    //         .await
    //         .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;
    // }

    let quiz = Quiz {
        id: ObjectId::new(),
        owner_id: user.id.clone(),
        title: json.title.clone(),
        description: json.description.clone(),
        questions,
        created_at: now.clone(),
        updated_at: now,
        shared_with: Vec::new(),
    };

    data.quiz_col
        .insert_one(&quiz)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;
    Ok(HttpResponse::Created().json(&quiz))
}
