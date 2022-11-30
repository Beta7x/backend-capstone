use mongodb::bson::{oid::ObjectId, Timestamp};
use serde::{ Serialize, Deserialize };

#[derive(Debug, Serialize, Deserialize)]
pub struct Article {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub title: String,
    pub description: String,
    pub image_url: String,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}