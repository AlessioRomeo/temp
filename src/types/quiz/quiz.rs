use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use crate::types::SharedWith;
use crate::MongoDateTime;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Question {
    pub prompt: String,
    pub options: [String; 4],
    pub correct_answer: char,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Quiz {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub owner_id: ObjectId,
    pub title: String,
    pub description: Option<String>,
    #[serde(default = "Vec::new")]
    pub questions: Vec<Question>,
    pub created_at: MongoDateTime,
    pub updated_at: MongoDateTime,
    pub shared_with: Vec<SharedWith>,
}


