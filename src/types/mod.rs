pub mod user;
mod board;
pub(crate) mod ws;
mod quiz;

pub use board::*;
pub use ws::*;
pub use quiz::*;

use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use bson::serde_helpers::serialize_object_id_as_hex_string;
pub use user::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct SharedWith {
    #[serde(serialize_with  = "serialize_object_id_as_hex_string")]
    pub user_id: ObjectId,
    pub can_update: bool,
}