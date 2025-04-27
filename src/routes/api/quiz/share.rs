
use actix_web::{web, HttpRequest, HttpResponse, Result as ActixResult};
use mongodb::bson::{doc, oid::ObjectId, DateTime as MongoDateTime};
use serde::Deserialize;
use crate::config::AppState;
use crate::routes::get_user_from_token;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ShareAction {
    Share,
    Revoke,
}

#[derive(Debug, Deserialize)]
pub struct ShareQuizData {
    pub usernames: Vec<String>,
    pub action: ShareAction,
    pub can_update: Option<bool>,
}

pub async fn share_quiz(
    data: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<String>,
    json: web::Json<ShareQuizData>,
) -> ActixResult<HttpResponse> {
    let me = get_user_from_token(&data, &req).await?;
    let quiz_id = ObjectId::parse_str(&path.into_inner())
        .map_err(|_| actix_web::error::ErrorBadRequest("Invalid quiz id"))?;

    let quiz = data.quiz_col
        .find_one(doc!{ "_id": &quiz_id })
        .await
        .unwrap()
        .ok_or_else(|| actix_web::error::ErrorNotFound("Quiz not found"))?;
    if quiz.owner_id != me.id {
        return Err(actix_web::error::ErrorForbidden("Not the owner"));
    }

    let mut target_ids = Vec::new();
    for username in &json.usernames {
        let user = data.user_col
            .find_one(doc!{ "username": username })
            .await
            .unwrap()
            .ok_or_else(|| actix_web::error::ErrorNotFound(format!("User `{}` not found", username)))?;
        target_ids.push(user.id);
    }

    let now = MongoDateTime::now();
    let update_doc = match json.action {
        ShareAction::Share => {
            let can_update = json
                .can_update
                .ok_or_else(|| actix_web::error::ErrorBadRequest("`can_update` required when sharing"))?;
            let entries: Vec<_> = target_ids.iter()
                .map(|uid| doc! { "user_id": uid, "can_update": can_update })
                .collect();
            doc! {
                "$addToSet": { "shared_with": { "$each": entries } },
                "$set": { "updated_at": now }
            }
        }
        ShareAction::Revoke => doc! {
            "$pull": { "shared_with": { "user_id": { "$in": target_ids } } },
            "$set": { "updated_at": now }
        },
    };

    data.quiz_col
        .update_one(doc!{ "_id": &quiz_id }, update_doc)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;
    Ok(HttpResponse::Ok().finish())
}
