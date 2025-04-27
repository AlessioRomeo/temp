use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::Collection;
use mongodb::bson;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id", serialize_with = "bson::serde_helpers::serialize_object_id_as_hex_string")]
    pub id: ObjectId,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub profile_picture_url: Option<String>,
    pub email: String,
    pub password_hash: String,
}



impl User {
    pub async fn find_by_username(username: String, table: &Collection<User>) -> Result<Option<User>, mongodb::error::Error> {
        match table.find_one(doc! {"username" : username}).await {
            Ok(Some(user)) => Ok(Some(user)),
            Ok(None) => Ok(None),
            Err(e) => Err(e),
        }
    }
    
    pub async fn find_by_email(email: String, table: &Collection<User>) -> Result<Option<User>, mongodb::error::Error> {
        match table.find_one(doc! {"email" : email}).await {
            Ok(Some(user)) => Ok(Some(user)),
            Ok(None) => Ok(None),
            Err(e) => Err(e),
        }
    }
}



