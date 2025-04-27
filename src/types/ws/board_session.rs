use actix::{Actor, ActorContext, Addr, AsyncContext, Handler, StreamHandler, WrapFuture};
use actix_web::web;
use actix_web_actors::ws;
use bson::doc;
use bson::oid::ObjectId;
use serde_json::Value;
use uuid::Uuid;
use crate::config::AppState;
use crate::types::ws::{BoardServer, BroadcastOp, CanvasOp, Connect, Disconnect}; 
use actix::fut::ActorFutureExt;

pub struct BoardSession {
    id:        Uuid,            // optional unique session id
    board_id:  ObjectId,
    user_id:   ObjectId,        // for ACL checks
    srv_addr:  Addr<BoardServer>,
    app_state: web::Data<AppState>,
}

impl BoardSession {
    pub fn new(
        board_id: ObjectId,
        user_id: ObjectId,
        srv_addr: Addr<BoardServer>,
        app_state: web::Data<AppState>,
    ) -> Self {
        BoardSession {
            id: Uuid::new_v4(),
            board_id,
            user_id,
            srv_addr,
            app_state
        }
    }
}

impl Actor for BoardSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let app_state = self.app_state.clone();
        let board_id   = self.board_id.clone();
        let user_id    = self.user_id.clone();
        let srv_addr   = self.srv_addr.clone();

        ctx.spawn(
            async move {
                app_state
                    .board_col
                    .find_one(doc! { "_id": &board_id })
                    .await
            }
                .into_actor(self)            // now returns an ActorFuture
                .map(move |res, act, ctx| {
                    match res {
                        Ok(Some(board)) => {
                            let is_owner  = board.owner_id == user_id;
                            let is_shared = board
                                .shared_with
                                .iter()
                                .any(|sw| sw.user_id == user_id);
                            if is_owner || is_shared {
                                srv_addr.do_send(Connect {
                                    session_addr: ctx.address(),
                                    board_id,
                                });
                            } else {
                                ctx.stop();
                            }
                        }
                        _ => ctx.stop(),
                    }
                })
        );
    }


    fn stopped(&mut self, _ctx: &mut Self::Context) {
        // notify server we left
        self.srv_addr.do_send(Disconnect {
            session_addr: _ctx.address(),
            board_id: self.board_id.clone(),
        });
    }
}

// Incoming WS messages from the client
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for BoardSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(txt)) => {
                if let Ok(op) = serde_json::from_str::<Value>(&txt) {
                    // forward to server
                    self.srv_addr.do_send(CanvasOp {
                        board_id: self.board_id.clone(),
                        op,
                    });
                }
            }
            Ok(ws::Message::Close(_)) => {
                ctx.stop();
            }
            _ => {}
        }
    }
}

// When the server broadcasts an op to us…
impl Handler<BroadcastOp> for BoardSession {
    type Result = ();

    fn handle(&mut self, msg: BroadcastOp, ctx: &mut Self::Context) {
        // push it back down the WS to the client
        if let Ok(txt) = serde_json::to_string(&msg.0) {
            ctx.text(txt);
        }
    }
}
