use mongodb::bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    #[serde(rename = "_id")]
    pub id: ObjectId,

    pub sender_id: ObjectId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub receiver_id: Option<ObjectId>, 

    #[serde(skip_serializing_if = "Option::is_none")]
    pub room_id: Option<ObjectId>,     

    pub content: String,

    pub timestamp: DateTime,
}
