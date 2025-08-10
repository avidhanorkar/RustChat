use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use std::{clone::Clone, fmt::Debug};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Room {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub name: String,
    pub owner: ObjectId,
    pub participants: Vec<ObjectId>,
}
