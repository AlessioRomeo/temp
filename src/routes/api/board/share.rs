use actix_web::{web, HttpRequest, HttpResponse, Result as ActixResult};
use mongodb::bson::{doc, oid::ObjectId};
use serde::Deserialize;
use crate::config::AppState;
use crate::routes::get_user_from_token;
use crate::MongoDateTime;
#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ShareAction {
    Share,
    Revoke,
}

#[derive(Debug, Deserialize)]
pub struct ShareBoardData {
    /// One or more usernames to share/unshare
    pub usernames: Vec<String>,

    /// "share" = grant access | "revoke" = pull access
    pub action: ShareAction,

    /// only required when action == Share
    pub can_update: Option<bool>,
}

pub async fn share_board(
    data: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<String>,
    json: web::Json<ShareBoardData>,
) -> ActixResult<HttpResponse> {
    let me = get_user_from_token(&data, &req).await?;
    
    let board_id = ObjectId::parse_str(&path.into_inner())
        .map_err(|_| actix_web::error::ErrorBadRequest("Invalid board id"))?;
    let board = data.board_col
        .find_one(doc! { "_id": &board_id })
        .await
        .map_err(|_| actix_web::error::ErrorNotFound("Invalid board id"))?
        .ok_or_else(|| actix_web::error::ErrorNotFound("Board not found"))?;
    if board.owner_id != me.id {
        return Err(actix_web::error::ErrorForbidden("Not the owner"));
    }


    let mut target_ids = Vec::with_capacity(json.usernames.len());
    for username in &json.usernames {
        let user = data.user_col
            .find_one(doc! { "username": username })
            .await
            .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?
            .ok_or_else(|| actix_web::error::ErrorNotFound(format!("User `{}` not found", username)))?;
        target_ids.push(user.id);
    }

    let now = MongoDateTime::now();
    let update_doc = match json.action {
        ShareAction::Share => {
            let can_update = json
                .can_update
                .ok_or_else(|| actix_web::error::ErrorBadRequest("`can_update` required when sharing"))?;

            let entries: Vec<_> = target_ids
                .iter()
                .map(|uid| doc! {
                    "user_id": uid,
                    "can_update": can_update
                })
                .collect();

            doc! {
                "$addToSet": {
                    "shared_with": {
                        "$each": entries
                    },
                },
                "$set": {
                    "updated_at": now
                }   
            }
        }
        ShareAction::Revoke => {
            doc! {
                "$pull": {
                    "shared_with": {
                        "user_id": { "$in": target_ids }
                    }
                },
                "$set": {
                    "updated_at": now
                }  
            }
        }
    };


    data.board_col
        .update_one(doc! { "_id": &board_id }, update_doc)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    Ok(HttpResponse::Ok().finish())
}
