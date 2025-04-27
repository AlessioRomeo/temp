use std::collections::{HashMap, HashSet};
use actix::{Actor, Addr, Context, Handler};
use actix_web::web;
use bson::{doc, to_bson};
use bson::oid::ObjectId;
use crate::config::AppState;
use crate::types::ws::board_session::BoardSession;
use crate::types::ws::{BroadcastOp, CanvasOp, Connect, Disconnect};
use crate::MongoDateTime;

type Room = HashSet<Addr<BoardSession>>;

pub struct BoardServer {
    /// board_id → set of sessions
    rooms: HashMap<ObjectId, Room>,
    app_state: web::Data<AppState>,
}

impl BoardServer {
    pub fn new(app_state: web::Data<AppState>) -> Self {
        BoardServer { rooms: HashMap::new(), app_state }
    }
}

impl Actor for BoardServer {
    type Context = Context<Self>;
}

// Handle new connections
impl Handler<Connect> for BoardServer {
    type Result = ();

    fn handle(&mut self, msg: Connect, _ctx: &mut Self::Context) {
        self.rooms
            .entry(msg.board_id.clone())
            .or_default()
            .insert(msg.session_addr.clone());
    }
}

// Handle disconnections
impl Handler<Disconnect> for BoardServer {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _ctx: &mut Self::Context) {
        if let Some(sessions) = self.rooms.get_mut(&msg.board_id) {
            sessions.remove(&msg.session_addr);
            if sessions.is_empty() {
                self.rooms.remove(&msg.board_id);
            }
        }
    }
}

// Handle incoming ops: broadcast + persist
impl Handler<CanvasOp> for BoardServer {
    type Result = ();

    fn handle(&mut self, msg: CanvasOp, _ctx: &mut Self::Context) {
        // 1) Broadcast to everyone in the room
        if let Some(sessions) = self.rooms.get(&msg.board_id) {
            for sess in sessions {
                sess.do_send(BroadcastOp(msg.op.clone()));
            }
        }

        // 2) Persist to Mongo (fire-and-forget)
        let board_col = self.app_state.board_col.clone();
        let bid = msg.board_id.clone();
        let op = msg.op.clone();
        let bson_op = to_bson(&op)
            .expect("Failed to convert canvas op to BSON");
        actix_rt::spawn(async move {
            let now = MongoDateTime::now();
            let _ = board_col.update_one(
                doc! { "_id": bid.clone() },
                doc! {
                  "$push": { "canvas_operations": bson_op },
                  "$set":  { "updated_at": now }
                },
            ).await;
        });
    }
}


