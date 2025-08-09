use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use bson::{doc, oid::ObjectId};
use futures_util::stream::StreamExt;
use mongodb::{Collection, Database};
use serde::Serialize;
use std::sync::Arc;

//crates
use crate::models::user_model::User;

#[derive(Serialize)]
pub struct UserResponse {
    pub name: String,
    pub email: String,
}

pub async fn get_user_by_id(
    State(db): State<Arc<Database>>,
    Path(id): Path<String>,
) -> Result<Json<UserResponse>, (StatusCode, String)> {
    let collection: Collection<User> = db.collection("user");

    if id.is_empty() {
        return Err((StatusCode::BAD_REQUEST, "Id is not given".to_string()));
    }

    let obj_id = ObjectId::parse_str(&id)
        .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid ID format".to_string()))?;

    let filter = doc! {
        "_id": &obj_id
    };

    match collection.find_one(filter).await {
        Ok(Some(user_found)) => {
            return Ok(Json(UserResponse {
                name: user_found.name,
                email: user_found.email,
            }));
        }
        Ok(None) => {
            return Err((
                StatusCode::NOT_FOUND,
                "No user is present associated with the given id".to_string(),
            ));
        }
        Err(e) => {
            println!("Some err occured: {}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error".to_string(),
            ));
        }
    }
}

pub async fn get_all_user(
    State(db): State<Arc<Database>>,
) -> Result<Json<Vec<UserResponse>>, (StatusCode, String)> {
    let collection: Collection<User> = db.collection("user");

    let mut cursor = match collection.find(doc! {}).await {
        Ok(cursor) => cursor,
        Err(e) => {
            println!("Some Error in Database: {}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error".to_string(),
            ));
        }
    };

    let mut users = Vec::new();

    while let Some(result) = cursor.next().await {
        match result {
            Ok(user) => {
                users.push(UserResponse {
                    name: user.name,
                    email: user.email,
                });
            }
            Err(e) => {
                println!("Error in reading user: {}", e);
            }
        }
    }
    Ok(Json(users))
}

pub async fn search_by_name(
    State(db): State<Arc<Database>>,
    Path(name): Path<String>,
) -> Result<Json<Vec<UserResponse>>, (StatusCode, String)> {
    let collection: Collection<User> = db.collection("user");
    if name.is_empty() {
        return Err((StatusCode::BAD_REQUEST, "Name is not given".to_string()));
    }

    let filter = doc! {
        "name": &name
    };

    let mut cursor = match collection.find(filter).await {
        Ok(cursor) => cursor,
        Err(e) => {
            println!("Some error occured: {}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error".to_string(),
            ));
        }
    };

    let mut users = Vec::new();

    while let Some(result) = cursor.next().await {
        match result {
            Ok(user) => {
                users.push(UserResponse {
                    name: user.name,
                    email: user.email,
                });
            }
            Err(e) => {
                println!("Some Error occured: {}", e);
            }
        }
    }

    Ok(Json(users))
}

pub async fn delete_user(
    State(db): State<Arc<Database>>,
    Path(id): Path<String>,
) -> Result<String, (StatusCode, String)> {
    let collection: Collection<User> = db.collection("user");

    let obj_id = ObjectId::parse_str(id)
        .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid ID format".to_string()))?;

    match collection.find_one_and_delete(doc! {"_id": obj_id}).await {
        Ok(Some(user_found)) => return Ok(
            format!("The User with user id: {}, is deleted successfully", user_found.id)),
        Ok(None) => {
            println!("There are no users with this user id");
            return Err((StatusCode::NOT_FOUND, "User Not Found".to_string()));
        }
        Err(e) => {
            println!("Database Error: {}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error".to_string(),
            ));
        }
    }
}
