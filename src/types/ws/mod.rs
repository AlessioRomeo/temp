mod board_server;
mod board_session;

pub use board_server::*;

use actix::prelude::*;
use serde_json::Value;
use mongodb::bson::oid::ObjectId;
pub(crate) use crate::types::ws::board_session::BoardSession;

// Sent from session → server when a client joins
pub struct Connect {
    pub session_addr: Addr<BoardSession>,
    pub board_id:     ObjectId,
}
impl Message for Connect {
    type Result = ();
}

// Sent from session → server when a client leaves
pub struct Disconnect {
    pub session_addr: Addr<BoardSession>,
    pub board_id:     ObjectId,
}
impl Message for Disconnect {
    type Result = ();
}

// Sent from session → server when a client submits a canvas op
pub struct CanvasOp {
    pub board_id: ObjectId,
    pub op:       Value, // e.g. { type: "draw", path: [...] }
}
impl Message for CanvasOp {
    type Result = ();
}

// Sent from server → session to forward an op
#[derive(Message)]
#[rtype(result = "()")]
pub struct BroadcastOp(pub Value);


