use actix_web::{HttpRequest, HttpResponse, Error, web};
use actix_web_actors::ws;
use actix::Addr;
use bson::oid::ObjectId;

use crate::config::AppState;
use crate::routes::get_user_from_token;
use crate::types::ws::{BoardServer, BoardSession};

/// WebSocket upgrade endpoint for real-time board collaboration.
/// GET /ws/boards/{id}
pub async fn board_ws(
    req: HttpRequest,
    stream: web::Payload,
    data: web::Data<AppState>,
    srv: web::Data<Addr<BoardServer>>,
) -> Result<HttpResponse, Error> {
    // 1) Authenticate via Bearer token
    let user = get_user_from_token(&data, &req).await?;

    // 2) Extract and parse board ID from the path
    let id_str = req
        .match_info()
        .get("id")
        .ok_or_else(|| actix_web::error::ErrorBadRequest("Missing board id"))?;
    let board_id = ObjectId::parse_str(id_str)
        .map_err(|_| actix_web::error::ErrorBadRequest("Invalid board id"))?;

    // 3) Create a new WS session actor
    let session = BoardSession::new(
        board_id,
        user.id.clone(),
        srv.get_ref().clone(),
        data.clone(),
    );

    // 4) Hand off to Actix-Web’s WS handler
    ws::start(session, &req, stream)
}
