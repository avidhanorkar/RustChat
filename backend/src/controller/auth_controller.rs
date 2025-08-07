use axum::{Json, extract::State, http::StatusCode};
use bcrypt::{DEFAULT_COST, hash, verify};
use bson::{doc, oid::ObjectId};
use mongodb::{Collection, Database};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
// Crates
use crate::models::user_model::User;

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub msg: String,
    pub user_id: ObjectId,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String
}

pub async fn register(
    State(db): State<Arc<Database>>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, String)> {
    let collection: Collection<User> = db.collection("user");

    if payload.name.is_empty() || payload.email.is_empty() || payload.password.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            "Name, email and password are required".to_string(),
        ));
    }

    let filter = doc! {
        "email": &payload.email,
    };

    match collection.find_one(filter).await {
        Ok(Some(_)) => {
                return Err((StatusCode::BAD_REQUEST, "Email already exists".to_string()));
        }
        Ok(None) => {
            let hashed: String = match hash(&payload.password, DEFAULT_COST) {
                Ok(hashed) => hashed,
                Err(e) => {
                    println!("Some error occured in hashing the password: {}", e);
                    return Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Failed to hash password".to_string(),
                    ));
                }
            };

            let user = User {
                id: ObjectId::new(),
                name: payload.name,
                email: payload.email,
                password: hashed,
            };

            match collection.insert_one(&user).await {
                Ok(user_created) => {
                    return Ok(Json(AuthResponse {
                        msg: "User created Successfully".to_string(),
                        user_id: user_created.inserted_id.as_object_id().ok_or((
                            StatusCode::INTERNAL_SERVER_ERROR,
                            "Failed to get user id".to_string(),
                        ))?,
                    }));
                }
                Err(e) => {
                    println!("Some error occured in inserting the user: {}", e);
                    return Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Failed to insert user".to_string(),
                    ));
                }
            }
        }
        Err(e) => {
            println!("Some error occured in finding the user: {}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to find user".to_string(),
            ));
        }
    }
}


pub async fn login(
    State(db): State<Arc<Database>>,
    Json(payload): Json<LoginRequest>
) -> Result<Json<AuthResponse>, (StatusCode, String)> {

    let collection: Collection<User> = db.collection("user");

    if payload.email.is_empty() || payload.password.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            "Email and password cannot be empty".to_string(),
        ));
    }

    let filter = doc! { "email": payload.email };  

    match collection.find_one(filter).await {
        Ok(Some(user)) => {

            match verify(&payload.password, &user.password) {
                Ok(true) => {
                    let user_id = user.id;

                        return Ok(Json(AuthResponse { msg: "User Logged in Successfully".to_string(), user_id: user_id }));
                }
                Ok(false) => {
                    return Err((
                        StatusCode::UNAUTHORIZED,
                        "Invalid password".to_string(),
                        ));
                }
                Err(e) => {
                    println!("Error in finding the email: {}", e);
                    return Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Error in verifying the password".to_string(),
                    ));
                }
            }
        } Ok(None) => {
            return Err((
                StatusCode::NOT_FOUND,
                "Email not Found".to_string()
            ));
        }
        Err(e) => {
            println!("Database error while finding user: {}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error".to_string(),
            ))
        }
    }
}
