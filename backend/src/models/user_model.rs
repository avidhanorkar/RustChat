use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use std::{clone::Clone, fmt::Debug};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct User {
    #[serde(rename = "#id")]
    pub id: ObjectId,
    pub name: String,
    pub email: String,
    pub password: String,
}
