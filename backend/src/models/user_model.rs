use bson::oid::ObjectId;
use serde::Deserialize;
use std::{clone::Clone, fmt::Debug};

#[derive(Deserialize, Clone, Debug)]
pub struct User {
    #[serde(rename = "#id", skip_serializing_if = "Option::is_none")]
    pub id: ObjectId,
    pub name: String,
    pub email: String,
    pub password: String,
}
