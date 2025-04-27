use bson::serde_helpers::{
    serialize_object_id_as_hex_string,
    bson_datetime_as_rfc3339_string,
};
use crate::MongoDateTime;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use crate::types::SharedWith;

#[derive(Debug, Serialize, Deserialize)]
pub struct Board {
    #[serde(rename = "_id", serialize_with = "serialize_object_id_as_hex_string")]
    pub id: ObjectId,
    #[serde(rename = "owner_id", serialize_with = "serialize_object_id_as_hex_string")]
    pub owner_id: ObjectId,
    pub title: String,
    pub description: Option<String>,
    #[serde(default)]
    pub canvas_operations: Vec<serde_json::Value>,
    #[serde(default)]
    pub shared_with: Vec<SharedWith>,
    #[serde(serialize_with = "bson_datetime_as_rfc3339_string::serialize")]
    pub created_at: MongoDateTime,
    #[serde(serialize_with = "bson_datetime_as_rfc3339_string::serialize")]
    pub updated_at: MongoDateTime,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_owner: Option<bool>
}